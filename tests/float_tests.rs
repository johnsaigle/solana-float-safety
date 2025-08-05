use solana_floats::float_ops::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_addition() {
        let result = add_floats(3.14, 2.86);
        assert!((result - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_float_multiplication() {
        let result = multiply_floats(2.5, 4.0);
        assert!((result - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_float_division() {
        let result = divide_floats(10.0, 2.0).unwrap();
        assert!((result - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_float_division_by_zero() {
        let result = divide_floats(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_sqrt_positive() {
        let result = sqrt_float(16.0);
        assert!((result - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sqrt_negative() {
        let result = sqrt_float(-1.0);
        assert!(result.is_nan());
    }

    #[test]
    fn test_float_precision() {
        let a = 0.1_f32;
        let b = 0.2_f32;
        let result = add_floats(a, b);
        assert!((result - 0.3).abs() < 0.0001);
    }

    #[tokio::test]
    async fn test_program_float_add_instruction() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "solana_floats",
            program_id,
            processor!(solana_floats::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut instruction_data = vec![0u8]; // Add instruction
        instruction_data.extend_from_slice(&3.14_f32.to_le_bytes());
        instruction_data.extend_from_slice(&2.86_f32.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_program_float_multiply_instruction() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "solana_floats",
            program_id,
            processor!(solana_floats::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut instruction_data = vec![1u8]; // Multiply instruction
        instruction_data.extend_from_slice(&2.5_f32.to_le_bytes());
        instruction_data.extend_from_slice(&4.0_f32.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_program_float_divide_instruction() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "solana_floats",
            program_id,
            processor!(solana_floats::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut instruction_data = vec![2u8]; // Divide instruction
        instruction_data.extend_from_slice(&10.0_f32.to_le_bytes());
        instruction_data.extend_from_slice(&2.0_f32.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_program_divide_by_zero() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "solana_floats",
            program_id,
            processor!(solana_floats::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut instruction_data = vec![2u8]; // Divide instruction
        instruction_data.extend_from_slice(&10.0_f32.to_le_bytes());
        instruction_data.extend_from_slice(&0.0_f32.to_le_bytes());

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }
}