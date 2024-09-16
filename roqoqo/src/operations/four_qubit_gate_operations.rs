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

use crate::prelude::*;
use ndarray::Array2;
use num_complex::Complex64;

/// The triple-controlled PauliX gate.
///
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperateFourQubit,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TripleControlledPauliX {
    /// The first control qubit involved in the triple-controlled PauliX gate.
    control_0: usize,
    /// The second control qubit involved in the triple-controlled PauliX gate.
    control_1: usize,
    /// The third control qubit involved in the triple-controlled PauliX gate.
    control_2: usize,
    /// The target qubit to apply the PauliX gate to.
    target: usize,
}

impl super::ImplementedIn1point15 for TripleControlledPauliX {}

impl SupportedVersion for TripleControlledPauliX {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 15, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_TripleControlledPauliX: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "TripleControlledPauliX",
];

impl OperateGate for TripleControlledPauliX {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 16;
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        for i in 0..dim - 2 {
            array[(i, i)] = Complex64::new(1.0, 0.0);
        }
        array[(dim - 2, dim - 1)] = Complex64::new(1.0, 0.0);
        array[(dim - 1, dim - 2)] = Complex64::new(1.0, 0.0);
        Ok(array)
    }
}

/// The triple-controlled PauliZ gate.
///
///
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::Substitute,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct TripleControlledPauliZ {
    /// The first control qubit involved in the triple-controlled PauliZ gate.
    control_0: usize,
    /// The second control qubit involved in the triple-controlled PauliZ gate.
    control_1: usize,
    /// The third control qubit involved in the triple-controlled PauliZ gate.
    control_2: usize,
    /// The target qubit to apply the PauliZ gate to.
    target: usize,
}

impl super::ImplementedIn1point15 for TripleControlledPauliZ {}

impl SupportedVersion for TripleControlledPauliZ {
    fn minimum_supported_roqoqo_version(&self) -> (u32, u32, u32) {
        (1, 15, 0)
    }
}

#[allow(non_upper_case_globals)]
const TAGS_TripleControlledPauliZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "MultiQubitGateOperation",
    "TripleControlledPauliZ",
];

impl OperateGate for TripleControlledPauliZ {
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let dim = 16;
        let mut array: Array2<Complex64> = Array2::zeros((dim, dim));
        for i in 0..dim - 1 {
            array[(i, i)] = Complex64::new(1.0, 0.0);
        }
        array[(dim, dim)] = Complex64::new(-1.0, 0.0);
        Ok(array)
    }
}
