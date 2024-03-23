#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use tokenizers::tokenizer::Tokenizer as HuggingFaceTokenizer;

#[php_class(name = "Manticore\\Ext\\Tokenizer")]
struct Tokenizer {
	tokenizer: HuggingFaceTokenizer,
}

#[php_impl]
impl Tokenizer {
  /// Static method to instantiate the Tokenizer
  #[php_static_method]
  /// @param string $tokenizer_file Path to the json file of the tokenizer to use
  /// @return self Instance of created class
  pub fn create(tokenizer_file: String) -> Self {
      let tokenizer = HuggingFaceTokenizer::from_file(&tokenizer_file).unwrap();
      Tokenizer { tokenizer }
  }

  /// @param string $text Text to convert into the token and return array of it
  /// @return array<string>
	pub fn tokenize(&self, text: String) -> Vec<String> {
		let encoding = self.tokenizer.encode(text, false).unwrap();
		encoding.get_tokens().to_vec()
	}

  /// @param string $text Text to convert into list of ids according to tokenizer config
  /// @return array<int>
	pub fn encode(&self, text: String) -> Vec<u32> {
		let encoding = self.tokenizer.encode(text, false).unwrap();
		encoding.get_ids().to_vec()
	}
}


/// Module initialization
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
	module
}
