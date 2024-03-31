use tss_esapi::{
    abstraction::{ak, ek, AsymmetricAlgorithmSelection},
    interface_types::{
        algorithm::{HashingAlgorithm, SignatureSchemeAlgorithm},
        ecc::EccCurve,
        reserved_handles::Hierarchy,
        structure_tags::AttestationType,
    },
    structures::{Data, PcrSelectionListBuilder, PcrSlot, Public, SignatureScheme},
    Context, Error,
};

#[swtpm_test::test]
fn test_quote(context: &mut Context) {
    context
        .execute_with_nullauth_session::<_, _, Error>(|ctx| {
            let pcr_selection_list = PcrSelectionListBuilder::new()
                .with_selection(HashingAlgorithm::Sha256, &[PcrSlot::Slot7])
                .build()
                .expect("Failed to pcr selection list");

            let qualifying_data = vec![0xff; 16];

            let public = signing_key_pub(ctx);
            let key_handle = ctx
                .create_primary(Hierarchy::Platform, public, None, None, None, None)?
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
        })
        .unwrap();
}

pub fn signing_key_pub(context: &mut Context) -> Public {
    let ek_ecc = ek::create_ek_object(
        context,
        AsymmetricAlgorithmSelection::Ecc(EccCurve::NistP384),
        None,
    )
    .unwrap();
    ak::create_ak(
        context,
        ek_ecc,
        HashingAlgorithm::Sha256,
        AsymmetricAlgorithmSelection::Ecc(EccCurve::NistP256),
        SignatureSchemeAlgorithm::EcDsa,
        None,
        None,
    )
    .unwrap()
    .out_public
}
