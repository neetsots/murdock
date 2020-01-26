use std::io::Error;
use crate::buffer::Buffer;
use crate::header::Header;
use crate::question::Question;
use crate::querytype::QueryType;
use crate::record::Record;

#[derive(Clone, Debug)]
pub struct Packet {
  pub header: Header,
  pub questions: Vec<Question>,
  pub answers: Vec<Record>,
  pub authorities: Vec<Record>,
  pub resources: Vec<Record>
}

impl Packet {
  pub fn new() -> Packet {
    Packet {
      header: Header::new(),
      questions: Vec::new(),
      answers: Vec::new(),
      authorities: Vec::new(),
      resources: Vec::new(),
    }
  }

  pub fn from_buffer(buffer: &mut Buffer) -> Result<Packet, Error> {
    let mut result = Packet::new();

    for _ in 0..result.header.questions {
        let mut question = Question::new("".to_string(),
                                         QueryType::UNKNOWN(0));
        question.read(buffer)?;
        result.questions.push(question);
    }

    for _ in 0..result.header.answers {
        let rec = Record::read(buffer)?;
        result.answers.push(rec);
    }
    for _ in 0..result.header.authoritative_entries {
        let rec = Record::read(buffer)?;
        result.authorities.push(rec);
    }
    for _ in 0..result.header.resource_entries {
        let rec = Record::read(buffer)?;
        result.resources.push(rec);
    }

    Ok(result)
  }
}