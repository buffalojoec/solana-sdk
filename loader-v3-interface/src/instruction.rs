use {crate::reader::instr_rdr, solana_instruction_error::InstructionError};

pub const MINIMUM_EXTEND_PROGRAM_BYTES: u32 = 10_240;

pub enum UpgradeableLoaderInstruction {
    InitializeBuffer,
    Write,
    DeployWithMaxDataLen,
    Upgrade,
    SetAuthority,
    Close,
    ExtendProgram,
    SetAuthorityChecked,
}

impl UpgradeableLoaderInstruction {
    pub const fn read(data: &[u8]) -> Result<&Self, InstructionError> {
        match data.first() {
            Some(0) => Ok(&Self::InitializeBuffer),
            Some(1) => Ok(&Self::Write),
            Some(2) => Ok(&Self::DeployWithMaxDataLen),
            Some(3) => Ok(&Self::Upgrade),
            Some(4) => Ok(&Self::SetAuthority),
            Some(5) => Ok(&Self::Close),
            Some(6) => Ok(&Self::ExtendProgram),
            Some(7) => Ok(&Self::SetAuthorityChecked),
            _ => Err(InstructionError::InvalidInstructionData),
        }
    }

    pub const fn pack(self) -> [u8; 4] {
        (self as u32).to_le_bytes()
    }
}

pub struct WritePayload<'a> {
    pub offset: u32,
    pub bytes: &'a [u8],
}

impl<'a> WritePayload<'a> {
    pub fn read(data: &'a [u8]) -> Result<Self, InstructionError> {
        let mut rdr = instr_rdr();
        let offset = rdr.read_u32(data)?;
        let len = rdr.read_u64(data)? as usize;
        let bytes = rdr.read_slice(data)?;
        if bytes.len() != len {
            return Err(InstructionError::InvalidInstructionData);
        }
        Ok(Self { offset, bytes })
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 8 + self.bytes.len());
        out.extend_from_slice(&self.offset.to_le_bytes());
        out.extend_from_slice(&(self.bytes.len() as u64).to_le_bytes());
        out.extend_from_slice(self.bytes);
        out
    }
}

pub struct DeployWithMaxDataLenPayload {
    pub max_data_len: u64,
}

impl DeployWithMaxDataLenPayload {
    pub fn read(data: &[u8]) -> Result<Self, InstructionError> {
        let max_data_len = instr_rdr().read_u64(data)?;
        Ok(Self { max_data_len })
    }

    pub const fn pack(self) -> [u8; 8] {
        self.max_data_len.to_le_bytes()
    }
}

pub struct ExtendProgramPayload {
    pub additional_bytes: u32,
}

impl ExtendProgramPayload {
    pub fn read(data: &[u8]) -> Result<Self, InstructionError> {
        let additional_bytes = instr_rdr().read_u32(data)?;
        Ok(Self { additional_bytes })
    }

    pub const fn pack(self) -> [u8; 4] {
        self.additional_bytes.to_le_bytes()
    }
}
