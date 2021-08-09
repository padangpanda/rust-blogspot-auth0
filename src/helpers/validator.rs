use passwords::analyzer;
use validator::ValidationError;

pub fn contain_everything(password: &str) -> Result<(), ValidationError> {
    let tobe_checked = analyzer::analyze(password);
    let uppercase = tobe_checked.uppercase_letters_count();
    let lowercase = tobe_checked.lowercase_letters_count();
    let number = tobe_checked.numbers_count();

    if uppercase >= 1 && number >= 1 && lowercase >= 1 {
        return Ok(())
    }
    
    return Err(ValidationError::new("Password must contain uppercase lowercase and numbers!"));
}