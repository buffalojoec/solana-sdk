use {crate::reader::acct_rdr, solana_instruction_error::InstructionError, solana_pubkey::Pubkey};

pub enum UpgradeableLoaderState {
    Uninitialized,
    Buffer,
    Program,
    ProgramData,
}

impl UpgradeableLoaderState {
    pub const fn read(data: &[u8]) -> Result<&Self, InstructionError> {
        match data.first() {
            Some(0) => Ok(&Self::Uninitialized),
            Some(1) => Ok(&Self::Buffer),
            Some(2) => Ok(&Self::Program),
            Some(3) => Ok(&Self::ProgramData),
            _ => Err(InstructionError::InvalidAccountData),
        }
    }

    pub const fn pack(self) -> [u8; 4] {
        (self as u32).to_le_bytes()
    }
}

pub struct Buffer<'a> {
    pub authority_address: Option<Pubkey>,
    pub data: &'a [u8],
}

impl<'a> Buffer<'a> {
    const DISCRIMINATOR: u8 = 1;
    pub const METADATA_LEN: usize = 37;

    pub const fn new(authority_address: Option<Pubkey>, data: &'a [u8]) -> Self {
        Self {
            authority_address,
            data,
        }
    }

    pub fn read(data: &'a [u8]) -> Result<Self, InstructionError> {
        let mut rdr = acct_rdr();
        rdr.read_discriminator_checked(data, Self::DISCRIMINATOR)?;
        let authority_address = rdr.read_option_pubkey(data)?;
        let data = rdr.read_slice(data)?;
        Ok(Self::new(authority_address, data))
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut out = vec![0u8; Self::METADATA_LEN + self.data.len()];
        out[0] = Self::DISCRIMINATOR;
        pack_option_pubkey(&self.authority_address, &mut out[4..Self::METADATA_LEN]);
        out[Self::METADATA_LEN..].copy_from_slice(self.data);
        out
    }
}

pub struct Program {
    pub programdata_address: Pubkey,
}

impl Program {
    const DISCRIMINATOR: u8 = 2;
    pub const METADATA_LEN: usize = 36;

    pub const fn new(programdata_address: Pubkey) -> Self {
        Self {
            programdata_address,
        }
    }

    pub fn read(data: &[u8]) -> Result<Self, InstructionError> {
        let mut rdr = acct_rdr();
        rdr.read_discriminator_checked(data, Self::DISCRIMINATOR)?;
        let programdata_address = rdr.read_pubkey(data)?;
        Ok(Self::new(programdata_address))
    }

    pub fn pack(&self) -> [u8; Self::METADATA_LEN] {
        let mut out = [0u8; Self::METADATA_LEN];
        out[0] = Self::DISCRIMINATOR;
        out[4..].copy_from_slice(self.programdata_address.as_array());
        out
    }
}

pub struct ProgramData<'a> {
    pub slot: u64,
    pub upgrade_authority_address: Option<Pubkey>,
    pub data: &'a [u8],
}

impl<'a> ProgramData<'a> {
    const DISCRIMINATOR: u8 = 3;
    pub const METADATA_LEN: usize = 45;

    pub const fn new(slot: u64, upgrade_authority_address: Option<Pubkey>, data: &'a [u8]) -> Self {
        Self {
            slot,
            upgrade_authority_address,
            data,
        }
    }

    pub fn read(data: &'a [u8]) -> Result<Self, InstructionError> {
        let mut rdr = acct_rdr();
        rdr.read_discriminator_checked(data, Self::DISCRIMINATOR)?;
        let slot = rdr.read_u64(data)?;
        let upgrade_authority_address = rdr.read_option_pubkey(data)?;
        let data = rdr.read_slice(data)?;
        Ok(Self::new(slot, upgrade_authority_address, data))
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut out = vec![0u8; Self::METADATA_LEN + self.data.len()];
        out[0] = Self::DISCRIMINATOR;
        out[4..12].copy_from_slice(&self.slot.to_le_bytes());
        pack_option_pubkey(
            &self.upgrade_authority_address,
            &mut out[12..Self::METADATA_LEN],
        );
        out[Self::METADATA_LEN..].copy_from_slice(self.data);
        out
    }
}

fn pack_option_pubkey(authority: &Option<Pubkey>, out: &mut [u8]) {
    if let Some(pubkey) = authority {
        out[0] = 1;
        out[1..33].copy_from_slice(pubkey.as_array());
    }
}
