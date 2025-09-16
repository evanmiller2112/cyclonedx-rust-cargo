use cyclonedx_bom::models::bom::{Bom, SpecVersion};
use cyclonedx_bom::models::metadata::Metadata;

fn main() {
    println!("Testing CycloneDX v1.6 support...");

    // Create a simple BOM
    let bom = Bom {
        version: 1,
        metadata: Some(Metadata::default()),
        ..Bom::default()
    };

    // Test v1.6 JSON serialization
    let mut output = Vec::new();
    match bom.output_as_json(&mut output, SpecVersion::V1_6) {
        Ok(_) => {
            let json_str = String::from_utf8(output).unwrap();
            println!("✅ v1.6 JSON generation successful!");
            println!("Generated JSON contains specVersion: {}",
                json_str.contains("\"specVersion\": \"1.6\""));
        }
        Err(e) => {
            println!("❌ v1.6 JSON generation failed: {:?}", e);
        }
    }

    // Test v1.6 XML serialization
    let mut xml_output = Vec::new();
    match bom.output_as_xml(&mut xml_output, SpecVersion::V1_6) {
        Ok(_) => {
            let xml_str = String::from_utf8(xml_output).unwrap();
            println!("✅ v1.6 XML generation successful!");
            println!("Generated XML contains namespace: {}",
                xml_str.contains("http://cyclonedx.org/schema/bom/1.6"));
        }
        Err(e) => {
            println!("❌ v1.6 XML generation failed: {:?}", e);
        }
    }
}