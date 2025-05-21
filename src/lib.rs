use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// 声明程序的入口点
entrypoint!(process_instruction);

// 程序入口处理函数
pub fn process_instruction(
    program_id: &Pubkey,        // 程序ID
    accounts: &[AccountInfo],   // 账户列表
    instruction_data: &[u8],    // 指令数据
) -> ProgramResult {
    msg!("Rust-Solana程序启动");
    
    // 这里添加您的程序逻辑
    
    msg!("程序执行成功");
    Ok(())
} 