use {solana_instruction_error::InstructionError, solana_pubkey::Pubkey};

pub struct Reader {
    err: InstructionError,
    offset: usize,
}

impl Reader {
    pub fn read_array<const N: usize>(&mut self, data: &[u8]) -> Result<[u8; N], InstructionError> {
        let (head, _) = data
            .get(self.offset..)
            .and_then(|rest| rest.split_first_chunk::<N>())
            .ok_or_else(|| self.err())?;
        self.offset += N;
        Ok(*head)
    }

    pub fn read_u8(&mut self, data: &[u8]) -> Result<u8, InstructionError> {
        self.read_array::<1>(data).map(|[b]| b)
    }

    pub fn read_u32(&mut self, data: &[u8]) -> Result<u32, InstructionError> {
        self.read_array::<4>(data).map(u32::from_le_bytes)
    }

    pub fn read_u64(&mut self, data: &[u8]) -> Result<u64, InstructionError> {
        self.read_array::<8>(data).map(u64::from_le_bytes)
    }

    pub fn read_slice<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], InstructionError> {
        let rest = data.get(self.offset..).ok_or_else(|| self.err())?;
        self.offset = data.len();
        Ok(rest)
    }

    pub fn read_pubkey(&mut self, data: &[u8]) -> Result<Pubkey, InstructionError> {
        self.read_array::<32>(data).map(Pubkey::new_from_array)
    }

    pub fn read_option_pubkey(&mut self, data: &[u8]) -> Result<Option<Pubkey>, InstructionError> {
        let tag = self.read_u8(data)?;
        let pubkey = self.read_pubkey(data)?;
        Ok((tag != 0).then_some(pubkey))
    }

    pub fn read_discriminator_checked(
        &mut self,
        data: &[u8],
        expected: u8,
    ) -> Result<(), InstructionError> {
        if self.read_array::<4>(data)?[0] != expected {
            return Err(self.err());
        }
        Ok(())
    }

    fn err(&self) -> InstructionError {
        self.err.clone()
    }
}

pub fn acct_rdr() -> Reader {
    Reader {
        err: InstructionError::InvalidAccountData,
        offset: 0,
    }
}

pub fn instr_rdr() -> Reader {
    Reader {
        err: InstructionError::InvalidInstructionData,
        offset: 0,
    }
}
