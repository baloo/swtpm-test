# Test environment for TPM

This crate provides a test environment for testing rust code against a `swtpm` instance.

```rust
#[swtpm_test::test]
fn test_quote(context: &mut Context) {
    context.execute_with_nullauth_session::<_, _, Error>(|ctx| {
        let pcr_selection_list = PcrSelectionListBuilder::new()
            .with_selection(
                HashingAlgorithm::Sha256,
                &[
                    PcrSlot::Slot7,
                ],
            )
            .build()
            .expect("Failed to pcr selection list");

        let qualifying_data = vec![0xff; 16];

        let key_handle = ctx
            .create_primary(
                Hierarchy::Platform,
                signing_key_pub(),
                None,
                None,
                None,
                None,
            )?
            .key_handle;

        let (attest, _signature) = ctx.quote(
            key_handle,
            Data::try_from(qualifying_data).unwrap(),
            SignatureScheme::Null,
            pcr_selection_list.clone(),
        )?;
        assert_eq!(
            AttestationType::Quote,
            attest.attestation_type(),
            "Attestation type of the returned value is not indicating Quote"
        );

        Ok(())
    }).unwrap();
}
```

