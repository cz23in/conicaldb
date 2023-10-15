/*
 * This is used to provide an error interface which is
 * safe to disclose to clients.
 * This becomes safe by paritioning parts of the
 * error information between private and public.
 * This means that an error message can be displayed to
 * the client separate from what would appear in server logs.
 */
pub struct ConicalError {
	good:				bool,
	code:				u16,
	public_message:		&'static str,
	private_message:	&'static str	
}

impl ConicalError {
	pub fn non_error() -> Self {
		ConicalError {
			good: true,
			code: 0,
			public_message: "",
			private_message: ""
		}
	}
	
	pub fn error_builder(
		code: u16,
		public_message: &'static str,
		private_message: &'static str
	) -> Self {
		ConicalError {
			good: false,
			code: code,
			private_message: private_message,
			public_message: public_message
		}
	}
}