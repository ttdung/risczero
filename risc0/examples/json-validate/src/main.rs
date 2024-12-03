// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// use serde_json::json;
// use jsonschema::{Draft, JSONSchema};
// use json_validate_core::Outputs;
use json_validate_methods::{CHECK_SCHEMA_ELF,CHECK_SCHEMA_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::Write;

fn main() {
    // let data = include_str!("../res/data_complex_obj.json");
    // let schema = include_str!("../res/schema_complex_obj.json");

    // let data = include_str!("../res/data_array.json");
    // let schema = include_str!("../res/schema_array.json");

    let data = include_str!("../res/data.json");
    let schema = include_str!("../res/schema.json");

    let outputs = check_schema(data, schema);
    println!();
    println!("validate schema result {}", outputs);

    // benchmark_prove(data, schema);
}

fn check_schema(data: &str, schema: &str) -> u32 {
    let input = (data, schema);
    println!("data {}", data);
    println!("schema {}", schema);

    // Obtain the default prover.
    let prover = default_prover();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, CHECK_SCHEMA_ELF).unwrap().receipt;

    receipt.verify(CHECK_SCHEMA_ID).unwrap();

    // Dump receipe using serde
    let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
        
    // Write the JSON string to a file
    let mut file = File::create("./res/receipt.json").expect("failed to create file");
    file.write_all(receipt_json.as_bytes()).expect("failed to write");

    // println!("Data written to file successfully.");

    receipt.journal.decode().unwrap()

}

/*
fn benchmark_prove(data: &str, schema: &str) {
    // start benchmarks
    const ITER: usize = 3;
    let mut benches = Vec::new();
    let mut benches_verify = Vec::new();

    let input = (data, schema);
    for _ in 0..ITER {
        // Obtain the default prover.
        let prover = default_prover();

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let before = Instant::now();
        // Produce a receipt by proving the specified ELF binary.
        let receipt  = prover.prove(env, CHECK_SCHEMA_ELF).unwrap().receipt;
        // println!("\n###### Time: {:.2?}", before.elapsed());
        benches.push(before.elapsed());

        let before_verify = Instant::now();
        receipt.verify(CHECK_SCHEMA_ID).unwrap();
        benches_verify.push(before_verify.elapsed());
    }

    println!("\n-------- BENCHMARK VERIFY ---------");
    for bench in benches_verify {
        println!("{:.2?}", bench);
    }
    println!("\n---------------------------");
    

    println!("\n-------- BENCHMARK ---------");
    for bench in benches {
        println!("{:.2?}", bench);
    }
    println!("\n---------------------------");
    // end benchmarks

}
*/
#[cfg(test)]
mod tests {
    use crate::check_schema;
    #[test]
    fn success_case() {
        let data = include_str!("../res/data.json");
        let schema = include_str!("../res/schema.json");

        let outputs = check_schema(data, schema);
        assert_eq!(
            outputs.result, 1,
            "The input data is not satisfy the schema"
        );
    }
    #[test]
    fn fail_case() {
        let data = include_str!("../res/data_failcase.json");
        let schema = include_str!("../res/schema.json");

        let outputs = check_schema(data, schema);
        assert_eq!(outputs.result, 0, "The input data is satisfy the schema");
    }
}
