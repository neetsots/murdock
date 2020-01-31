use std::io::{Error, ErrorKind};

pub struct Buffer {
  pub buf: [u8; 512],
  pos: usize
}

impl Buffer {

  pub fn new() -> Buffer {
    Buffer { buf: [0; 512], pos: 0 }
  }

  fn get(&mut self, pos: usize) -> Result<u8, Error> {
    if pos >= 512 {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(self.buf[pos])
  }

  fn get_range(&mut self, pos: usize, len: usize) -> Result<&[u8], Error> {
    if pos + len >= 512 {
        return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    Ok(&self.buf[pos..pos+len as usize])
  }

  fn read_u8(&mut self) -> Result<u8, Error> {
    if self.pos >= 512 {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    let res = self.buf[self.pos];
    self.pos += 1;
    Ok(res)
  }

  pub fn read_u16(&mut self) -> Result<u16, Error> {
    let res = ((self.read_u8()? as u16) << 8) |
               (self.read_u8()? as u16);
    Ok(res)
  }

  pub fn read_u32(&mut self) -> Result<u32, Error> {
    let res = ((self.read_u16()? as u32) << 16) |
              (self.read_u16()? as u32);
    Ok(res)
  }

  pub fn write_u8(&mut self, val: u8) -> Result<(), Error> {
    if self.pos >= 512 {
      return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
    }
    self.buf[self.pos] = val;
    self.pos += 1;
    Ok(())
  }

  pub fn write_u16(&mut self, val: u16) -> Result<(), Error> {
    self.write_u8((val>>8) as u8)?;
    self.write_u8((val & 0xFF) as u8)?;
    Ok(())
  }

  pub fn write_u32(&mut self, val: u32) -> Result<(), Error> {
    self.write_u16((val>>16) as u16)?;
    self.write_u16((val & 0xFFFF) as u16)?;
    Ok(())
  }

  pub fn get_domain_name(&mut self, domain_name: &mut String) -> Result<(),Error> {
    let mut pos = self.pos;
    let mut delim = "";
    let mut jump = false;

    loop{
      let len = self.get(pos)?;
      pos += 1;

      if len == 0 {
        break;
      }
      else if (len & 0xC0) == 0xC0 {
          // Check if a jump is needed
          if !jump {
            self.pos = pos+1;
          }

          // Perform jump
          let byte2 = self.get(pos)? as u16;
          let offset = (((len as u16) ^ 0xC0) << 8) | byte2;
          pos = offset as usize;
          jump = true;
      }
      else {
        // Build domain name
        let word = self.get_range(pos, len as usize)?;

        domain_name.push_str(delim);
        domain_name.push_str(&String::from_utf8_lossy(word).to_lowercase());
        delim = ".";

        pos += len as usize;
      }
    }

    if !jump {
      self.pos = pos;
    }
    Ok(())
  }

  pub fn set_domain_name(&mut self, domain_name: &str) -> Result<(),Error> {
    let split_name = domain_name.split('.').collect::<Vec<&str>>();

    for label in split_name {
      let label_len = label.len();
      if label_len > 0x34 {
        return Err(Error::new(ErrorKind::InvalidInput, "Single label exceeds 63 characters"));
      }
      else {
        self.write_u8(label_len as u8)?;
        for byte in label.as_bytes() {
          self.write_u8(*b)?;
        }
      }
    }
    self.write_u8(0);
    Ok (())
  }
}