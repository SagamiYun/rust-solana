use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};

// 定义计数器指令类型
#[derive(Debug, PartialEq)]
pub enum CounterInstruction {
    // 初始化计数器账户，从0开始
    Initialize,
    // 增加计数器的值
    Increment,
    // 减少计数器的值
    Decrement,
}

// 定义计数器状态结构
#[derive(Debug, Default)]
pub struct Counter {
    pub is_initialized: bool,
    pub count: u32,
}

// 实现Pack trait以便序列化和反序列化
impl Sealed for Counter {}

impl IsInitialized for Counter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Counter {
    const LEN: usize = 5; // 1 byte for is_initialized + 4 bytes for count (u32)

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let is_initialized = src[0] != 0;
        let count = u32::from_le_bytes([src[1], src[2], src[3], src[4]]);

        Ok(Counter {
            is_initialized,
            count,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        let count_bytes = self.count.to_le_bytes();
        dst[1..5].copy_from_slice(&count_bytes);
    }
}

// 解析指令数据
fn unpack_instruction_data(instruction_data: &[u8]) -> Result<CounterInstruction, ProgramError> {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(match instruction_data[0] {
        0 => CounterInstruction::Initialize,
        1 => CounterInstruction::Increment,
        2 => CounterInstruction::Decrement,
        _ => return Err(ProgramError::InvalidInstructionData),
    })
}

// 处理初始化指令
fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;

    // 确保账户没有被初始化
    if counter_account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter_info = Counter::unpack_unchecked(&counter_account.data.borrow())?;
    if counter_info.is_initialized {
        msg!("Counter account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // 初始化计数器
    counter_info.is_initialized = true;
    counter_info.count = 0;

    // 保存数据前先记录值，避免移动后使用错误
    let count = counter_info.count;
    Counter::pack(counter_info, &mut counter_account.data.borrow_mut())?;
    
    msg!("Counter account initialized with count: {}", count);
    Ok(())
}

// 处理增加计数器值的指令
fn process_increment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;

    // 确保账户属于当前程序
    if counter_account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter_info = Counter::unpack(&counter_account.data.borrow())?;
    
    // 增加计数
    counter_info.count = counter_info.count.checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // 保存数据前先记录值，避免移动后使用错误
    let count = counter_info.count;
    Counter::pack(counter_info, &mut counter_account.data.borrow_mut())?;
    
    msg!("Counter incremented to: {}", count);
    Ok(())
}

// 处理减少计数器值的指令
fn process_decrement(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;

    // 确保账户属于当前程序
    if counter_account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter_info = Counter::unpack(&counter_account.data.borrow())?;
    
    // 减少计数，但不能小于0
    if counter_info.count == 0 {
        msg!("Counter cannot be decremented below 0");
        return Err(ProgramError::InvalidArgument);
    }
    
    counter_info.count = counter_info.count.checked_sub(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // 保存数据前先记录值，避免移动后使用错误
    let count = counter_info.count;
    Counter::pack(counter_info, &mut counter_account.data.borrow_mut())?;
    
    msg!("Counter decremented to: {}", count);
    Ok(())
}

// 声明程序的入口点
entrypoint!(process_instruction);

// 程序入口处理函数
pub fn process_instruction(
    program_id: &Pubkey,        // 程序ID
    accounts: &[AccountInfo],   // 账户列表
    instruction_data: &[u8],    // 指令数据
) -> ProgramResult {
    msg!("Counter程序启动");
    
    // 解析指令
    let instruction = unpack_instruction_data(instruction_data)?;
    
    // 根据指令类型调用相应的处理函数
    match instruction {
        CounterInstruction::Initialize => process_initialize(program_id, accounts),
        CounterInstruction::Increment => process_increment(program_id, accounts),
        CounterInstruction::Decrement => process_decrement(program_id, accounts),
    }
}