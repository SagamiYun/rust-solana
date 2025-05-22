use solana_sdk::signature::read_keypair_file;
use {
    solana_client::rpc_client::RpcClient,
    solana_program::{
        instruction::Instruction,
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        native_token::LAMPORTS_PER_SOL,
        signature::Signature,
        signer::{keypair::Keypair, Signer},
        transaction::Transaction,
    },
    std::{error::Error, str::FromStr},
};

use rust_solana::Counter;


fn main() -> Result<(), Box<dyn Error>> {
    println!("启动Solana计数器客户端...");

    // 连接到本地Solana测试网络
    let rpc_url = String::from("http://localhost:8899");
    let connection = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    println!("连接到Solana测试网络: {}", rpc_url);
    
    // 从文件加载钱包，如果文件不存在则创建新钱包
    let payer = match read_keypair_file("wallet-keypair.json") {
        Ok(keypair) => {
            println!("使用已存在的钱包: {}", keypair.pubkey());
            keypair
        },
        Err(_) => {
            println!("未找到钱包文件，创建新钱包");
            let new_keypair = Keypair::new();

            // 请求空投SOL代币用于支付交易费
            request_airdrop(&connection, &new_keypair.pubkey(), 2.0)?;
            println!("已为新钱包空投 2 SOL");

            // 等待空投确认
            std::thread::sleep(std::time::Duration::from_secs(3));

            new_keypair
        }
    };

    // 检查钱包余额
    let balance = connection.get_balance(&payer.pubkey())?;
    println!("当前钱包余额: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
    

    // 请求空投SOL代币用于支付交易费
    // request_airdrop(&connection, &payer.pubkey(), 2.0)?;
    // println!("已为测试钱包空投 2 SOL");

    // 加载或创建计数器程序ID
    let program_id_str = "EnKfzEUyaAxGSmFbhD4yezLZ7tXMoQRPcNYVg2Xxi2Cj";
    let program_id = match Pubkey::from_str(program_id_str) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            println!("无效的程序ID: {}，请替换为有效的程序ID", program_id_str);
            return Err("无效的程序ID".into());
        }
    };
    println!("使用程序ID: {}", program_id);

    // 为计数器创建一个新的账户密钥对
    let counter_keypair = Keypair::new();
    let counter_pubkey = counter_keypair.pubkey();

    println!("创建计数器账户: {}", counter_pubkey);

    // 计算账户需要的空间
    let counter_space = Counter::LEN;

    // 计算账户所需的租金
    let rent = connection.get_minimum_balance_for_rent_exemption(counter_space)?;

    // 创建用于创建计数器账户的指令
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &counter_pubkey,
        rent,
        counter_space as u64,
        &program_id,
    );

    // 创建用于初始化计数器的指令
    let initialize_ix = Instruction {
        program_id,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(counter_pubkey, false),
        ],
        data: vec![0], // CounterInstruction::Initialize
    };

    // 获取最近的区块哈希
    let recent_blockhash = connection.get_latest_blockhash()?;

    // 创建交易，包括创建账户和初始化两个指令
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_keypair],
        recent_blockhash,
    );

    // 发送并确认交易
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("计数器初始化交易成功: {}", signature),
        Err(err) => {
            println!("计数器初始化交易失败: {}", err);
            return Err(Box::new(err));
        }
    }

    // 休息一下，确保交易被确认
    // std::thread::sleep(std::time::Duration::from_secs(2));

    // 增加计数器
    println!("\n执行增加计数器操作...");
    increment_counter(&connection, &payer, &program_id, &counter_pubkey)?;

    // 休息一下，确保交易被确认
    // std::thread::sleep(std::time::Duration::from_secs(2));

    // 再次增加计数器
    println!("\n再次执行增加计数器操作...");
    increment_counter(&connection, &payer, &program_id, &counter_pubkey)?;

    // 休息一下，确保交易被确认
    // std::thread::sleep(std::time::Duration::from_secs(2));

    // 减少计数器
    println!("\n执行减少计数器操作...");
    decrement_counter(&connection, &payer, &program_id, &counter_pubkey)?;

    // 获取并显示当前计数
    match connection.get_account_data(&counter_pubkey) {
        Ok(data) => {
            match Counter::unpack(&data) {
                Ok(counter) => println!("\n当前计数: {}", counter.count),
                Err(err) => println!("解析计数器数据失败: {}", err),
            }
        },
        Err(err) => println!("获取计数器账户数据失败: {}", err),
    }

    println!("\n计数器演示完成！");
    Ok(())
}

fn increment_counter(
    connection: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    counter_pubkey: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    // 创建增加计数的指令
    let increment_ix = Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*counter_pubkey, false),
        ],
        data: vec![1], // CounterInstruction::Increment
    };

    // 获取最近的区块哈希
    let recent_blockhash = connection.get_latest_blockhash()?;

    // 创建交易
    let transaction = Transaction::new_signed_with_payer(
        &[increment_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // 发送并确认交易
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("增加计数器交易成功: {}", signature),
        Err(err) => {
            println!("增加计数器交易失败: {}", err);
            return Err(Box::new(err));
        }
    }

    Ok(())
}

fn decrement_counter(
    connection: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    counter_pubkey: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    // 创建减少计数的指令
    let decrement_ix = Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*counter_pubkey, false),
        ],
        data: vec![2], // CounterInstruction::Decrement
    };

    // 获取最近的区块哈希
    let recent_blockhash = connection.get_latest_blockhash()?;

    // 创建交易
    let transaction = Transaction::new_signed_with_payer(
        &[decrement_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // 发送并确认交易
    match connection.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("减少计数器交易成功: {}", signature),
        Err(err) => {
            println!("减少计数器交易失败: {}", err);
            return Err(Box::new(err));
        }
    }

    Ok(())
}

// 请求空投SOL代币
fn request_airdrop(
    connection: &RpcClient,
    pubkey: &Pubkey,
    amount: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = connection.request_airdrop(pubkey, (amount * LAMPORTS_PER_SOL as f64) as u64)?;
    connection.confirm_transaction(&sig)?;
    Ok(sig)
}