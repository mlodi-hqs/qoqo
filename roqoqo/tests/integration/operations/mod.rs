// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

mod define_operations;

mod measurement_operations;

mod pragma_operations;

mod single_qubit_gate_operations;

mod two_qubit_gate_operations;

mod three_qubit_gate_operations;

mod four_qubit_gate_operations;

mod multi_qubit_gate_operations;

mod involved_classical;

mod supported_version;

mod bosonic_operations;

mod spin_boson_operations;

#[cfg(feature = "unstable_analog_operations")]
mod analog_operations;

use nalgebra as na;
use ndarray::Array2;
use num_complex::Complex64;
use roqoqo::operations::AVAILABLE_GATES_HQSLANG;

// Helper function to convert a two-dimensional ndarray to a NxM matrix (N, M depending on the vector)
// The output can be used to be converted into a nalgebra matrix with `na::Matrix4::from()`
// for a 4x4 matrix or `na::DMatrix::from()` for a more general matrix
pub fn convert_matrix(customarray: Array2<Complex64>) -> na::DMatrix<Complex64> {
    let dim = customarray.dim();
    na::DMatrix::<Complex64>::from_iterator(dim.0, dim.1, customarray.t().iter().cloned())
}

// Test InvolvedQubits clone
#[test]
fn test_involved_qubits_clone() {
    let iq = roqoqo::operations::InvolvedQubits::All;
    #[allow(clippy::redundant_clone)]
    let iq2 = iq.clone();
    assert_eq!(iq, iq2);
    let iq3 = roqoqo::operations::InvolvedQubits::None;
    let helper = iq != iq3;
    assert!(helper);
}

#[test]
fn test_available_gates() {
    assert!(AVAILABLE_GATES_HQSLANG.contains(&"Hadamard"));
    assert!(!AVAILABLE_GATES_HQSLANG.contains(&"Error"));
}
