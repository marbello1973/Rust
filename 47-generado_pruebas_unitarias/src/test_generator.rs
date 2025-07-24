use std::fs;
use std::path::Path;
use syn::parse::Parser;

pub fn generate_test(component_name: &str, props: Vec<String>, output_dir: &str){
    
    let test_content = format!(

    r#"
        import {{render}} from '@testing-library/react';
        import {0} from './{0}';
        test('renders {0} correctly', () => {{
                const {{ getBytext }} = render(<{0} {1} />);
                expect(getBytext('Render test')).toBeInTheDocument();
            }});
    "#,

    component_name,
    props
        .iter()
        .map(|prop| format!("{}='test'", prop))
        .collect::<Vect<String>>()
        .join(" ")
    );

    let test_file_path = Path::new(output_dir).join(format!("{component_name}.test.js"));
    fs::write(test_file_path, test_content).expect("Unable to write test file");
        
}
