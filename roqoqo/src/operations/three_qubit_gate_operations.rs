// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
// use crate::Circuit;
use ndarray::{array, Array2};
use num_complex::Complex64;

use qoqo_calculator::CalculatorFloat;

#[cfg(feature = "overrotate")]
use rand_distr::{Distribution, Normal};

/// Implements the double-controlled PauliZ gate.
///
/// The double-controlled PauliZ applies a PauliZ unitary to the `target` qubit
/// depending on the states of both `control_0` and `control_1` qubits.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlledControlledPauliZ {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the qubit PauliZ is applied to.
    target: usize,
}

#[allow(non_upper_case_globals)]
const TAGS_ControlledControlledPauliZ: &[&str; 4] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "ControlledControlledPauliZ",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for ControlledControlledPauliZ {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0)
            ],
        ])
    }
}

impl OperateThreeQubitGate for ControlledControlledPauliZ {
    fn circuit(&self) -> crate::Circuit {
        todo!()
    }
}

/// Implements the double-controlled PhaseShift gate.
///
/// The double-controlled PhaseShift applies a phase shift to the `target` qubit
/// depending on the states of both `control_0` and `control_1` qubits.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::InvolveQubits,
    roqoqo_derive::SupportedVersion,
    roqoqo_derive::Operate,
    roqoqo_derive::Substitute,
    roqoqo_derive::OperateThreeQubit,
    roqoqo_derive::Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlledControlledPhaseShift {
    /// The index of the most significant qubit in the unitary representation. Here, the first controlling qubit of the operation.
    control_0: usize,
    /// The index of the second most significant qubit in the unitary representation. Here, the second controlling qubit of the operation.
    control_1: usize,
    /// The index of the least significant qubit in the unitary representation. Here, the qubit the phase-shift is applied to.
    target: usize,
    /// The rotation angle θ.
    theta: CalculatorFloat,
}

#[allow(non_upper_case_globals)]
const TAGS_ControlledControlledPhaseShift: &[&str; 5] = &[
    "Operation",
    "GateOperation",
    "ThreeQubitGateOperation",
    "Rotation",
    "ControlledControlledPhaseShift",
];

/// Trait for all Operations acting with a unitary gate on a set of qubits.
impl OperateGate for ControlledControlledPhaseShift {
    /// Returns unitary matrix of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(Array2<Complex64>)` - The unitary matrix representation of the gate.
    /// * `Err(RoqoqoError)` - The conversion of parameters to f64 failed (here, not possible).
    fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
        let c: f64 = (f64::try_from(self.theta.clone())?).cos();
        let s: f64 = (f64::try_from(self.theta.clone())?).sin();
        Ok(array![
            [
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0)
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(c, s)
            ],
        ])
    }
}

impl OperateThreeQubitGate for ControlledControlledPhaseShift {
    fn circuit(&self) -> crate::Circuit {
        todo!()
    }
}
