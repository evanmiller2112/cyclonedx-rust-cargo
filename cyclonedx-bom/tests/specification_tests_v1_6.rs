mod v1_6 {
    use cyclonedx_bom::models::bom::{Bom, SpecVersion};
    use cyclonedx_bom::validation::Validate;
    use test_utils::validate_json_with_schema;

    #[test]
    fn it_should_parse_all_of_the_valid_xml_specifications() {
        insta::with_settings!({
            snapshot_path => "spec/snapshots/1.6",
            prepend_module_to_snapshot => false,
        }, {
            insta::glob!("spec/1.6/valid*.xml", |path| {
                let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
                let bom = Bom::parse_from_xml_v1_6(file).unwrap_or_else(|e| panic!("Failed to parse the document as an BOM: {path:?} {:#?}", e));

                let validation_result = bom.validate_version(SpecVersion::V1_6);
                if !validation_result.passed() {
                    dbg!(&validation_result);
                }
                assert!(
                    validation_result.passed(),
                    "{path:?} unexpectedly failed validation"
                );

                let mut output = Vec::new();
                bom.output_as_xml_v1_6(&mut output)
                    .unwrap_or_else(|_| panic!("Failed to output the file: {path:?}"));
                let bom_output = String::from_utf8_lossy(&output).to_string();

                insta::assert_snapshot!(bom_output);
            });
        });
    }

    #[test]
    fn it_should_parse_all_of_the_valid_json_specifications() {
        insta::with_settings!({
            snapshot_path => "spec/snapshots/1.6",
            prepend_module_to_snapshot => false,
        }, {
            insta::glob!("spec/1.6/valid*.json", |path| {
                let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
                let bom = Bom::parse_from_json_v1_6(file).unwrap_or_else(|e| panic!("Failed to parse the document as an BOM: {path:?} {:#?}", e));

                let validation_result = bom.validate_version(SpecVersion::V1_6);
                assert!(
                    validation_result.passed(),
                    "{path:?} unexpectedly failed validation"
                );

                let mut output = Vec::new();
                bom.output_as_json_v1_6(&mut output)
                    .unwrap_or_else(|_| panic!("Failed to output the file: {path:?}"));
                let bom_output = String::from_utf8_lossy(&output).to_string();

                insta::assert_snapshot!(bom_output);
            });
        });
    }

    #[test]
    fn it_should_validate_the_generated_json_schemas() {
        insta::glob!("spec/1.6/valid*.json", |path| {
            let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));

            validate_json_with_schema(file, "../specification-master/schema/bom-1.6.schema.json").unwrap_or_else(|e| panic!("Failed to validate schema: {path:?} {e}"));
        });
    }

    #[test]
    fn it_should_output_valid_xml_files() {
        insta::glob!("spec/1.6/valid*.json", |path| {
            let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
            let bom = Bom::parse_from_json_v1_6(file).unwrap_or_else(|e| panic!("Failed to parse the document as an BOM: {path:?} {:#?}", e));

            let mut output = Vec::new();
            bom.output_as_xml_v1_6(&mut output)
                .unwrap_or_else(|_| panic!("Failed to output the file: {path:?}"));

            // Ensure the output is valid UTF-8
            String::from_utf8(output).unwrap_or_else(|e| panic!("Invalid UTF-8 in output: {path:?} {e}"));
        });
    }

    #[test]
    fn it_should_output_valid_json_files() {
        insta::glob!("spec/1.6/valid*.xml", |path| {
            let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
            let bom = Bom::parse_from_xml_v1_6(file).unwrap_or_else(|e| panic!("Failed to parse the document as an BOM: {path:?} {:#?}", e));

            let mut output = Vec::new();
            bom.output_as_json_v1_6(&mut output)
                .unwrap_or_else(|_| panic!("Failed to output the file: {path:?}"));

            // Ensure the output is valid UTF-8
            let json_output = String::from_utf8(output).unwrap_or_else(|e| panic!("Invalid UTF-8 in output: {path:?} {e}"));

            // Ensure the output is valid JSON
            serde_json::from_str::<serde_json::Value>(&json_output).unwrap_or_else(|e| panic!("Invalid JSON in output: {path:?} {e}"));
        });
    }
}