// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use ndarray::Array2;
use num_complex::Complex64;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    BogoliubovWrapper, CNOTWrapper, ComplexPMInteractionWrapper, ControlledPauliYWrapper,
    ControlledPauliZWrapper, ControlledPhaseShiftWrapper, ControlledXWrapper, ControlledXYWrapper,
    FSwapWrapper, FsimWrapper, GivensRotationLittleEndianWrapper, GivensRotationWrapper,
    ISwapWrapper, InvSqrtISwapWrapper, MolmerSorensenXXWrapper, PMInteractionWrapper,
    PhaseShiftedControlledPhaseWrapper, PhaseShiftedControlledZWrapper, QsimWrapper, SWAPWrapper,
    SpinInteractionWrapper, SqrtISwapWrapper, VariableMSXXWrapper, XYWrapper,
};

use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
use roqoqo::RoqoqoError;
use std::collections::HashMap;
use std::convert::TryInto;
use test_case::test_case;

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

/// Test is_parametrized = false for TwoQubitGate Operations
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(!bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
        .unwrap());
    })
}

/// Test tags() function for TwoQubitGate Operations
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "CNOT",
        ],
    Operation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SWAP",
        ],
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ISwap",
        ],
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "FSwap",
        ],
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SqrtISwap",
        ],
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "InvSqrtISwap",
        ],
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "XY",
        ],
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "ControlledPhaseShift",
        ],
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledPauliY",
        ],
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledPauliZ",
        ],
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "MolmerSorensenXX",
        ],
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "VariableMSXX",
        ],
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "GivensRotation",
        ],
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Rotation",
        "GivensRotationLittleEndian",
        ],
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Qsim",
        ],
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Fsim",
        ],
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "SpinInteraction",
        ],
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "Bogoliubov",
        ],
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PMInteraction",
        ],
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ComplexPMInteraction",
        ],
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PhaseShiftedControlledZ",
        ],
    Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "PhaseShiftedControlledPhase",
        ],
    Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledX",
        ],
    Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "TwoQubitGateOperation",
        "ControlledXY",
        ],
    Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_tags(tags: Vec<&str>, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let tags_op: Vec<String> =
            Vec::<String>::extract(operation.call_method0(py, "tags").unwrap().as_ref(py)).unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

/// Test hqslang() function for TwoQubitGate Operations
#[test_case("CNOT", Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case("SWAP", Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case("ISwap", Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case("FSwap", Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case("SqrtISwap", Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case("InvSqrtISwap", Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case("XY", Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(
    "ControlledPhaseShift", 
    Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4))
; "ControlledPhaseShift")]
#[test_case("ControlledPauliY", Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case("ControlledPauliZ", Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case("MolmerSorensenXX", Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case("VariableMSXX", Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case("GivensRotation", Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case("GivensRotationLittleEndian", Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case("Qsim", Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case("Fsim", Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case("SpinInteraction", Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case("Bogoliubov", Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case("PMInteraction", Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case("ComplexPMInteraction", Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case("PhaseShiftedControlledZ", Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case("PhaseShiftedControlledPhase", Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case("ControlledX", Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case("ControlledXY", Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String =
            String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

/// Test remap_qubits() function for TwoQubitGate Operations
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();

        // test initial qubits
        let control: usize =
            usize::extract(operation.call_method0(py, "control").unwrap().as_ref(py)).unwrap();
        assert_eq!(control.clone(), 0);
        let target: usize =
            usize::extract(operation.call_method0(py, "target").unwrap().as_ref(py)).unwrap();
        assert_eq!(target.clone(), 1);

        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        qubit_mapping.insert(1, 3);
        qubit_mapping.insert(3, 1);
        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();

        // test re-mapped qubit
        let control_new: usize =
            usize::extract(result.call_method0(py, "control").unwrap().as_ref(py)).unwrap();
        assert_eq!(control_new.clone(), 2);
        let target_new: usize =
            usize::extract(result.call_method0(py, "target").unwrap().as_ref(py)).unwrap();
        assert_eq!(target_new.clone(), 3);

        // test that initial and rempapped qubits are different
        assert_ne!(control, control_new);
        assert_ne!(target, target_new);
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test unitary_matrix() function for TwoQubitGate Operations for the error case
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("test"))); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("test"))); "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("test"))); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("test"))); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("test"))); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::from("test"))); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from("test"))); "ControlledXY")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix");
        let result_ref = py_result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test unitary_matrix() function for TwoQubitGate Operations
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix").unwrap();
        let result_matrix = py_result
            .downcast::<PyArray2<Complex64>>(py)
            .unwrap()
            .to_owned_array();

        // compare to reference matrix obtained in Rust directly (without passing to Python)
        let gate: GateOperation = input_operation.try_into().unwrap();
        let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
        let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

        assert_eq!(result_matrix, test_matrix);
    })
}

/// Test format and repr functions
#[test_case(
    "CNOT { control: 0, target: 1 }",
    Operation::from(CNOT::new(0, 1)) ; "CNOT")
]
#[test_case(
    "SWAP { control: 1, target: 0 }",
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    "ISwap { control: 1, target: 0 }",
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    "FSwap { control: 1, target: 0 }",
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    "SqrtISwap { control: 1, target: 0 }",
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    "InvSqrtISwap { control: 1, target: 0 }",
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    "XY { control: 1, target: 0, theta: Float(3.141592653589793) }",
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    "ControlledPhaseShift { control: 1, target: 0, theta: Float(0.7853981633974483) }",
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    "ControlledPauliY { control: 1, target: 0 }",
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    "ControlledPauliZ { control: 1, target: 0 }",
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    "MolmerSorensenXX { control: 1, target: 0 }",
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    "VariableMSXX { control: 1, target: 0, theta: Float(3.141592653589793) }",
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    "GivensRotation { control: 1, target: 0, theta: Float(0.0), phi: Float(0.0) }",
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::ZERO, CalculatorFloat::ZERO)); "GivensRotation")]
#[test_case(
    "GivensRotationLittleEndian { control: 1, target: 0, theta: Float(0.0), phi: Float(0.0) }",
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::ZERO, CalculatorFloat::ZERO)); "GivensRotationLittleEndian")]
#[test_case(
    "Qsim { control: 1, target: 0, x: Float(1.0), y: Float(1.0), z: Float(-1.0) }",
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    "Fsim { control: 1, target: 0, t: Float(1.0), u: Float(2.0), delta: Float(-1.0) }",
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    "SpinInteraction { control: 1, target: 0, x: Float(1.0), y: Float(2.0), z: Float(-1.0) }",
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    "Bogoliubov { control: 1, target: 0, delta_real: Float(1.0), delta_imag: Float(-1.0) }",
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    "PMInteraction { control: 1, target: 0, t: Float(3.141592653589793) }",
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    "ComplexPMInteraction { control: 1, target: 0, t_real: Float(1.0), t_imag: Float(-1.0) }",
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    "PhaseShiftedControlledZ { control: 0, target: 1, phi: Float(3.141592653589793) }",
    Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(
    "PhaseShiftedControlledPhase { control: 0, target: 1, theta: Float(3.141592653589793), phi: Float(3.141592653589793) }",
    Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::PI)); "PhaseShiftedControlledPhase")]
#[test_case(
    "ControlledX { control: 0, target: 1, theta: Float(3.141592653589793) }",
    Operation::from(ControlledX::new(0, 1, CalculatorFloat::PI)); "ControlledX")]
#[test_case(
    "ControlledXY { control: 0, target: 1, theta: Float(3.141592653589793), phi: Float(3.141592653589793) }",
    Operation::from(ControlledXY::new(0, 1, CalculatorFloat::PI, CalculatorFloat::PI)); "ControlledXY")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::FRAC_PI_4)); "ControlledXY")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract(
            copy_op
                .as_ref(py)
                .call_method1("__eq__", (copy_deepcopy_param.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract(
            deepcopy_op
                .as_ref(py)
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test substitute_parameters function
#[test_case(Operation::from(CNOT::new(0, 1)),
            Operation::from(CNOT::new(0, 1));
            "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)),
            Operation::from(SWAP::new(0, 1));
            "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)),
            Operation::from(ISwap::new(0, 1));
            "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)),
            Operation::from(FSwap::new(0, 1));
            "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)),
            Operation::from(SqrtISwap::new(0, 1));
            "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)),
            Operation::from(InvSqrtISwap::new(0, 1));
            "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(XY::new(0, 1, CalculatorFloat::from(1.0)));
            "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(1.0)));
            "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)),
            Operation::from(ControlledPauliY::new(0, 1));
            "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)),
            Operation::from(ControlledPauliZ::new(0, 1));
            "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)),
            Operation::from(MolmerSorensenXX::new(0, 1));
            "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(1.0)));
            "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0))),
            Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0)));
            "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0))),
            Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0)));
            "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))),
            Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))),
            Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))),
            Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0))),
            Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0)));
            "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(1.0)));
            "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0))),
            Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0)));
            "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(1.0)));
            "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::PI)),
            Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::PI));
            "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::from("test"))),
            Operation::from(ControlledX::new(0, 1, CalculatorFloat::from(1.0)));
            "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::PI)),
            Operation::from(ControlledXY::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::PI));
            "ControlledXY")]
fn test_pyo3_substitute_parameters(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();
        let mut substitution_dict: HashMap<&str, f64> = HashMap::new();
        substitution_dict.insert("test", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = convert_operation_to_pyobject(second_op).unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters returning an error
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from("test"))); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from("test")));
            "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from("test")));
            "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0)));
            "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0)));
            "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0)));
            "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0)));
            "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from("test")));
            "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::from(0.0)));
            "ComplexPMInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from("test")));
            "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::PI));
            "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::from("test"))); "ControlledX")]
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::from("test"), CalculatorFloat::PI)); "ControlledXY")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<&str, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test powercf function of TwoQubitGate Operations
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from(0.005))),
            Operation::from(XY::new(0, 1, CalculatorFloat::from(0.005 * 1.5)));
            "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(0.005))),
            Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(0.005 * 1.5)));
            "ControlledPhaseShift")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(0.005))),
            Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(0.005 * 1.5)));
            "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02)));
            "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))),
            Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(0.005 * 1.5), CalculatorFloat::from(0.02)));
            "GivensRotationLittleEndian")]
fn test_pyo3_powercf(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();

        let power = convert_cf_to_pyobject(py, CalculatorFloat::from(1.5));
        let comparison_op = convert_operation_to_pyobject(second_op).unwrap();

        let remapped_op = operation.call_method1(py, "powercf", (power,)).unwrap();
        let comparison = bool::extract(
            remapped_op
                .call_method1(py, "__eq__", (comparison_op,))
                .unwrap()
                .as_ref(py),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test new() function for CNOT
#[test_case(Operation::from(CNOT::new(0, 1)), (0, 1,), "__eq__"; "CNOT_eq")]
#[test_case(Operation::from(CNOT::new(2, 1)), (0, 1,), "__ne__"; "CNOT_ne")]
fn test_new_cnot(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<CNOTWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<CNOTWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<CNOTWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<CNOTWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<CNOTWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "CNOTWrapper { internal: CNOT { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for SWAP
#[test_case(Operation::from(SWAP::new(0, 1)), (0, 1,), "__eq__"; "SWAP_eq")]
#[test_case(Operation::from(SWAP::new(2, 1)), (0, 1,), "__ne__"; "SWAP_ne")]
fn test_new_swap(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SWAPWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<SWAPWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SWAPWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<SWAPWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SWAPWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SWAPWrapper { internal: SWAP { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for ISwap
#[test_case(Operation::from(ISwap::new(0, 1)), (0, 1,), "__eq__"; "ISwap_eq")]
#[test_case(Operation::from(ISwap::new(2, 1)), (0, 1,), "__ne__"; "ISwap_ne")]
fn test_new_iswap(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<ISwapWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ISwapWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<ISwapWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<ISwapWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ISwapWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ISwapWrapper { internal: ISwap { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for FSwap
#[test_case(Operation::from(FSwap::new(0, 1)), (0, 1,), "__eq__"; "FSwap_eq")]
#[test_case(Operation::from(FSwap::new(2, 1)), (0, 1,), "__ne__"; "FSwap_ne")]
fn test_new_fswap(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<FSwapWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<FSwapWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<FSwapWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<FSwapWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<FSwapWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "FSwapWrapper { internal: FSwap { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for SqrtISwap
#[test_case(Operation::from(SqrtISwap::new(0, 1)), (0, 1,), "__eq__"; "SqrtISwap_eq")]
#[test_case(Operation::from(SqrtISwap::new(2, 1)), (0, 1,), "__ne__"; "SqrtISwap_ne")]
fn test_new_sqrtiswap(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SqrtISwapWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<SqrtISwapWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SqrtISwapWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<SqrtISwapWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SqrtISwapWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SqrtISwapWrapper { internal: SqrtISwap { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for InvSqrtISwap
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)), (0, 1,), "__eq__"; "InvSqrtISwap_eq")]
#[test_case(Operation::from(InvSqrtISwap::new(2, 1)), (0, 1,), "__ne__"; "InvSqrtISwap_ne")]
fn test_new_invsqrtiswap(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvSqrtISwapWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<InvSqrtISwapWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvSqrtISwapWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<InvSqrtISwapWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvSqrtISwapWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvSqrtISwapWrapper { internal: InvSqrtISwap { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for ControlledPauliY
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), (0, 1,), "__eq__"; "ControlledPauliY_eq")]
#[test_case(Operation::from(ControlledPauliY::new(2, 1)), (0, 1,), "__ne__"; "ControlledPauliY_ne")]
fn test_new_controlledpauliy(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<ControlledPauliYWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledPauliYWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<ControlledPauliYWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<ControlledPauliYWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ControlledPauliYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledPauliYWrapper { internal: ControlledPauliY { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for ControlledPauliZ
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), (0, 1,), "__eq__"; "ControlledPauliZ_eq")]
#[test_case(Operation::from(ControlledPauliZ::new(2, 1)), (0, 1,), "__ne__"; "ControlledPauliZ_ne")]
fn test_new_controlledpauliz(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<ControlledPauliZWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledPauliZWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<ControlledPauliZWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<ControlledPauliZWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ControlledPauliZWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledPauliZWrapper { internal: ControlledPauliZ { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for MolmerSorensenXX
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), (0, 1,), "__eq__"; "MolmerSorensenXX_eq")]
#[test_case(Operation::from(MolmerSorensenXX::new(2, 1)), (0, 1,), "__ne__"; "MolmerSorensenXX_ne")]
fn test_new_molmersorensenxx(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<MolmerSorensenXXWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<MolmerSorensenXXWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<MolmerSorensenXXWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2))
            .unwrap()
            .downcast::<PyCell<MolmerSorensenXXWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<MolmerSorensenXXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "MolmerSorensenXXWrapper { internal: MolmerSorensenXX { control: 1, target: 2 } }"
        );
    })
}

/// Test new() function for XY
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "XY_eq")]
#[test_case(Operation::from(XY::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "XY_ne")]
fn test_new_xy(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<XYWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<XYWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<XYWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<XYWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<XYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "XYWrapper { internal: XY { control: 1, target: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for ControlledPhaseShift
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "ControlledPhaseShift_eq")]
#[test_case(Operation::from(ControlledPhaseShift::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "ControlledPhaseShift_ne")]
fn test_new_controlledphaseshift(
    input_operation: Operation,
    arguments: (u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ControlledPhaseShiftWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledPhaseShiftWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<ControlledPhaseShiftWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<ControlledPhaseShiftWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ControlledPhaseShiftWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledPhaseShiftWrapper { internal: ControlledPhaseShift { control: 1, target: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for VariableMSXX
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "VariableMSXX_eq")]
#[test_case(Operation::from(VariableMSXX::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "VariableMSXX_ne")]
fn test_new_variablemsxx(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<VariableMSXXWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<VariableMSXXWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<VariableMSXXWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<VariableMSXXWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<VariableMSXXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "VariableMSXXWrapper { internal: VariableMSXX { control: 1, target: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for PMInteraction
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "PMInteraction_eq")]
#[test_case(Operation::from(PMInteraction::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "PMInteraction_ne")]
fn test_new_pminteraction(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PMInteractionWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PMInteractionWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<PMInteractionWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<PMInteractionWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PMInteractionWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PMInteractionWrapper { internal: PMInteraction { control: 1, target: 2, t: Float(0.0) } }"
        );
    })
}

/// Test new() function for GivensRotation
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "GivensRotation_eq")]
#[test_case(Operation::from(GivensRotation::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "GivensRotation_ne")]
fn test_new_givensrotation(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<GivensRotationWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<GivensRotationWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<GivensRotationWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<GivensRotationWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<GivensRotationWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "GivensRotationWrapper { internal: GivensRotation { control: 1, target: 2, theta: Float(0.0), phi: Float(0.0) } }"
        );
    })
}

/// Test new() function for GivensRotationLittleEndian
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "GivensRotationLittleEndian_eq")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "GivensRotationLittleEndian_ne")]
fn test_new_givensrotationlittleendian(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<GivensRotationLittleEndianWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<GivensRotationLittleEndianWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<GivensRotationLittleEndianWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<GivensRotationLittleEndianWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<GivensRotationLittleEndianWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "GivensRotationLittleEndianWrapper { internal: GivensRotationLittleEndian { control: 1, target: 2, theta: Float(0.0), phi: Float(0.0) } }"
        );
    })
}

/// Test new() function for Bogoliubov
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "Bogoliubov_eq")]
#[test_case(Operation::from(Bogoliubov::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "Bogoliubov_ne")]
fn test_new_bogoliubov(input_operation: Operation, arguments: (u32, u32, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<BogoliubovWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<BogoliubovWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<BogoliubovWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<BogoliubovWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<BogoliubovWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "BogoliubovWrapper { internal: Bogoliubov { control: 1, target: 2, delta_real: Float(0.0), delta_imag: Float(0.0) } }"
        );
    })
}

/// Test new() function for ComplexPMInteraction
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "ComplexPMInteraction_eq")]
#[test_case(Operation::from(ComplexPMInteraction::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "ComplexPMInteraction_ne")]
fn test_new_complexpminteraction(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ComplexPMInteractionWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ComplexPMInteractionWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<ComplexPMInteractionWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<ComplexPMInteractionWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ComplexPMInteractionWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ComplexPMInteractionWrapper { internal: ComplexPMInteraction { control: 1, target: 2, t_real: Float(0.0), t_imag: Float(0.0) } }"
        );
    })
}

/// Test new() function for Qsim
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__eq__"; "Qsim_eq")]
#[test_case(Operation::from(Qsim::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__ne__"; "Qsim_ne")]
fn test_new_qsim(input_operation: Operation, arguments: (u32, u32, f64, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<QsimWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<QsimWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0, 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<QsimWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<QsimWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<QsimWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "QsimWrapper { internal: Qsim { control: 1, target: 2, x: Float(0.0), y: Float(0.0), z: Float(0.0) } }"
        );
    })
}

/// Test new() function for Fsim
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__eq__"; "Fsim_eq")]
#[test_case(Operation::from(Fsim::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__ne__"; "Fsim_ne")]
fn test_new_fsim(input_operation: Operation, arguments: (u32, u32, f64, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<FsimWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<FsimWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0, 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<FsimWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<FsimWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<FsimWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "FsimWrapper { internal: Fsim { control: 1, target: 2, t: Float(0.0), u: Float(0.0), delta: Float(0.0) } }"
        );
    })
}

/// Test new() function for SpinInteraction
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__eq__"; "SpinInteraction_eq")]
#[test_case(Operation::from(SpinInteraction::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0, 0.0), "__ne__"; "SpinInteraction_ne")]
fn test_new_spininteraction(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<SpinInteractionWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<SpinInteractionWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], 0.0, 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, vec!["fails"], 0.0));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, 1, 0.0, 0.0, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<SpinInteractionWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<SpinInteractionWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SpinInteractionWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SpinInteractionWrapper { internal: SpinInteraction { control: 1, target: 2, x: Float(0.0), y: Float(0.0), z: Float(0.0) } }"
        );
    })
}

/// Test new() function for PhaseShiftedControlledZ
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "PhaseShiftedControlledZ_eq")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "PhaseShiftedControlledZ_ne")]
fn test_new_phaseshiftedcontrolledz(
    input_operation: Operation,
    arguments: (u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PhaseShiftedControlledZWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PhaseShiftedControlledZWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<PhaseShiftedControlledZWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<PhaseShiftedControlledZWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<PhaseShiftedControlledZWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftedControlledZWrapper { internal: PhaseShiftedControlledZ { control: 1, target: 2, phi: Float(0.0) } }"
        );
    })
}

/// Test new() function for PhaseShiftedControlledPhase
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "PhaseShiftedControlledPhase_eq")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "PhaseShiftedControlledPhase_ne")]
fn test_new_phaseshiftedcontrolledphase(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialization, no errors
        let operation_type = py.get_type::<PhaseShiftedControlledPhaseWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<PhaseShiftedControlledPhaseWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<PhaseShiftedControlledPhaseWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<PhaseShiftedControlledPhaseWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<PhaseShiftedControlledPhaseWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftedControlledPhaseWrapper { internal: PhaseShiftedControlledPhase { control: 1, target: 2, theta: Float(0.0), phi: Float(0.0) } }"
        );
    })
}

/// Test new() function for ControlledX
#[test_case(Operation::from(ControlledX::new(0, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__eq__"; "ControlledX_eq")]
#[test_case(Operation::from(ControlledX::new(2, 1, CalculatorFloat::from(0.0))), (0, 1, 0.0), "__ne__"; "ControlledX_ne")]
fn test_new_controlledx(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialization, no errors
        let operation_type = py.get_type::<ControlledXWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledXWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<ControlledXWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0))
            .unwrap()
            .downcast::<PyCell<ControlledXWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ControlledXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledXWrapper { internal: ControlledX { control: 1, target: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for ControlledXY
#[test_case(Operation::from(ControlledXY::new(0, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__eq__"; "ControlledXY_eq")]
#[test_case(Operation::from(ControlledXY::new(2, 1, CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), (0, 1, 0.0, 0.0), "__ne__"; "ControlledXY_ne")]
fn test_new_controlledxy(
    input_operation: Operation,
    arguments: (u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialization, no errors
        let operation_type = py.get_type::<ControlledXYWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledXYWrapper>>()
            .unwrap();
        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"], vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<ControlledXYWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 0.0, 0.0))
            .unwrap()
            .downcast::<PyCell<ControlledXYWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ControlledXYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledXYWrapper { internal: ControlledXY { control: 1, target: 2, theta: Float(0.0), phi: Float(0.0) } }"
        );
    })
}

/// Test the __richcmp__ function
#[test_case(
    Operation::from(CNOT::new(0, 1)),
    Operation::from(CNOT::new(1, 0)); "CNOT")]
#[test_case(
    Operation::from(SWAP::new(0, 1)),
    Operation::from(SWAP::new(1, 0)); "SWAP")]
#[test_case(
    Operation::from(ISwap::new(0, 1)),
    Operation::from(ISwap::new(1, 0)); "ISwap")]
#[test_case(
    Operation::from(FSwap::new(0, 1)),
    Operation::from(FSwap::new(1, 0)); "FSwap")]
#[test_case(
    Operation::from(SqrtISwap::new(0, 1)),
    Operation::from(SqrtISwap::new(1, 0)); "SqrtISwap")]
#[test_case(
    Operation::from(InvSqrtISwap::new(0, 1)),
    Operation::from(InvSqrtISwap::new(1, 0)); "InvSqrtISwap")]
#[test_case(
    Operation::from(XY::new(0, 1, CalculatorFloat::PI)),
    Operation::from(XY::new(1, 0, CalculatorFloat::PI)); "XY")]
#[test_case(
    Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)),
    Operation::from(ControlledPhaseShift::new(1, 0, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(
    Operation::from(ControlledPauliY::new(0, 1)),
    Operation::from(ControlledPauliY::new(1, 0)); "ControlledPauliY")]
#[test_case(
    Operation::from(ControlledPauliZ::new(0, 1)),
    Operation::from(ControlledPauliZ::new(1, 0)); "ControlledPauliZ")]
#[test_case(
    Operation::from(MolmerSorensenXX::new(0, 1)),
    Operation::from(MolmerSorensenXX::new(1, 0)); "MolmerSorensenXX")]
#[test_case(
    Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)),
    Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(
    Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    Operation::from(GivensRotation::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(
    Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)),
    Operation::from(GivensRotationLittleEndian::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(
    Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(Qsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(
    Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    Operation::from(Fsim::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(
    Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))),
    Operation::from(SpinInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(
    Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(Bogoliubov::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(
    Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)),
    Operation::from(PMInteraction::new(1, 0, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(
    Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))),
    Operation::from(ComplexPMInteraction::new(1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(
    Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)),
    Operation::from(PhaseShiftedControlledZ::new(1, 0, CalculatorFloat::PI)); "PhaseShiftedControlledZ")]
#[test_case(
    Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)),
    Operation::from(PhaseShiftedControlledPhase::new(1, 0, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledPhase")]
#[test_case(
    Operation::from(ControlledX::new(0, 1, CalculatorFloat::FRAC_PI_2)),
    Operation::from(ControlledX::new(1, 0, CalculatorFloat::FRAC_PI_2)); "ControlledX")]
#[test_case(
    Operation::from(ControlledXY::new(0, 1, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)),
    Operation::from(ControlledXY::new(1, 0, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "ControlledXY")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2).unwrap();

        let comparison = bool::extract(
            operation_one
                .as_ref(py)
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract(
            operation_one
                .as_ref(py)
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1(py, "__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1(py, "__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}
