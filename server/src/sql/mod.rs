use crate::conical_error::ConicalError;

pub struct SQL {}

impl SQL {
	pub fn parse() -> Result<SQL, ConicalError> {
		Ok(SQL {})	
	}
}