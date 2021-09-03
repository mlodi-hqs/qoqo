// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Integration test for public API of single qubit gate operations

//use crate::RoqoqoError::{CalculatorError, UnitaryMatrixErrror};
use nalgebra as na;
use ndarray::Array2;
use num_complex::Complex64;
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorError::FloatSymbolicNotConvertable;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::RoqoqoError;
use roqoqo::RoqoqoError::{CalculatorError, QubitMappingError, UnitaryMatrixErrror};
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::f64::consts::PI;
use std::f64::EPSILON;
use test_case::test_case;

/// Test SingleQubitGate alpha, beta, global phase
#[test_case(
    0,
    CalculatorFloat::from("alpha_r"),
    CalculatorFloat::from("alpha_i"),
    CalculatorFloat::from("beta_r"),
    CalculatorFloat::from("beta_i"),
    CalculatorFloat::from("global_phase");
    "symbolic")]
#[test_case(
    0,
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0);
    "all0")]
#[test_case(
    0,
    CalculatorFloat::from(1.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(1.0),
    CalculatorFloat::from(PI);
    "values")]
fn test_singlequbitgate_abp(
    qubit: usize,
    alpha_r: CalculatorFloat,
    alpha_i: CalculatorFloat,
    beta_r: CalculatorFloat,
    beta_i: CalculatorFloat,
    global_phase: CalculatorFloat,
) {
    let gate = SingleQubitGate::new(
        qubit.clone(),
        alpha_r.clone(),
        alpha_i.clone(),
        beta_r.clone(),
        beta_i.clone(),
        global_phase.clone(),
    );
    // verify that alpha, beta, global_phase functions return the passed value
    assert_eq!(gate.qubit(), &qubit);
    assert_eq!(gate.alpha_r(), alpha_r);
    assert_eq!(gate.alpha_i(), alpha_i);
    assert_eq!(gate.beta_r(), beta_r);
    assert_eq!(gate.beta_i(), beta_i);
    assert_eq!(gate.global_phase(), global_phase);
}

/// Test unitary matrix for SingleQubitGate
#[test_case(
    0,
    CalculatorFloat::from(1.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0);
    "alpha=1")]
#[test_case(
    0,
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(-1.0),
    CalculatorFloat::from(PI);
    "PI")]
fn test_singlequbitgate_unitarity_ok(
    qubit: usize,
    alpha_r: CalculatorFloat,
    alpha_i: CalculatorFloat,
    beta_r: CalculatorFloat,
    beta_i: CalculatorFloat,
    global_phase: CalculatorFloat,
) {
    let gate = SingleQubitGate::new(
        qubit.clone(),
        alpha_r.clone(),
        alpha_i.clone(),
        beta_r.clone(),
        beta_i.clone(),
        global_phase.clone(),
    );
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    assert!(result.is_ok());
    let result_matrix: Array2<Complex64> = result.unwrap();
    // check unitarity with nalgebra
    // convert ndarray into nalgebra matrix
    let result_matrix2 = na::Matrix2::new(
        result_matrix[[0, 0]],
        result_matrix[[0, 1]],
        result_matrix[[1, 0]],
        result_matrix[[1, 1]],
    );
    // calculate matrix product A*A_dagger
    let product = result_matrix2 * result_matrix2.adjoint(); //associated function 'self.conjugate_transpose()'  renamed 'self.adjoint()'.

    // convert complex matrix product into real matrix by taking the absolute value of the complex number, which should be sufficient if the matrix is unitary.
    let matrix_norm: na::Matrix2<f64> = na::Matrix2::new(
        product[0].norm_sqr(),
        product[1].norm_sqr(),
        product[2].norm_sqr(),
        product[3].norm_sqr(),
    );
    let epsilon = 1e-12;
    assert!(matrix_norm.is_identity(epsilon));
}

/// Test unitary matrix for SingleQubitGate if result is an Error since alpha and beta are both zero
#[test]
fn test_singlequbitgate_unitarity_err0() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
    );
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    assert!(result.is_err());
    // test error
    assert_eq!(
        result,
        Err(UnitaryMatrixErrror {
            alpha_r: 0.0,
            alpha_i: 0.0,
            beta_r: 0.0,
            beta_i: 0.0,
            norm: 0.0
        })
    );
    // test debugging
    assert_eq!(format!("{:?}", result), "Err(UnitaryMatrixErrror { alpha_r: 0.0, alpha_i: 0.0, beta_r: 0.0, beta_i: 0.0, norm: 0.0 })");
}

/// Test unitary matrix for SingleQubitGate if result is an Error since matrix is not normalized
#[test]
fn test_singlequbitgate_unitarity_err2() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(-1.0),
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
    );
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    assert!(result.is_err());
    // test error
    assert_eq!(
        result,
        Err(UnitaryMatrixErrror {
            alpha_r: 0.0,
            alpha_i: -1.0,
            beta_r: 1.0,
            beta_i: 0.0,
            norm: 2.0
        })
    );
    // test debugging
    assert_eq!(format!("{:?}", result), "Err(UnitaryMatrixErrror { alpha_r: 0.0, alpha_i: -1.0, beta_r: 1.0, beta_i: 0.0, norm: 2.0 })");
}

/// Test unitary matrix for SingleQubitGate if result is an CalculatorError
#[test]
fn test_singlequbitgate_unitarity_err() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from("alpha_r"),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
    );
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    assert!(result.is_err());
    // test error
    assert_eq!(
        result,
        Err(CalculatorError(FloatSymbolicNotConvertable {
            val: "alpha_r".to_string()
        }))
    );
    // test debugging
    assert_eq!(
        format!("{:?}", result),
        "Err(CalculatorError(FloatSymbolicNotConvertable { val: \"alpha_r\" }))"
    );
}

/// Test 'qubit()' for SingleQubitGate
#[test]
fn test_singlequbitgate_operatesinglequbit() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
    );
    let qubit_p: &usize = &gate.qubit();
    assert_eq!(qubit_p, &0);
}

/// Test 'clone()' for SingleQubitGate
#[test]
fn test_singlequbitgate_clone() {
    let gate1 = SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
    );
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

/// Test (De-)Serialization of SingleQubitGate
#[cfg(feature = "serialize")]
#[test]
fn ser_de_singlequbitgate() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    );
    assert_tokens(
        &gate.readable(),
        &[
            Token::Struct {
                name: "SingleQubitGate",
                len: 6,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("alpha_r"),
            Token::F64(0.0),
            Token::Str("alpha_i"),
            Token::F64(0.0),
            Token::Str("beta_r"),
            Token::F64(0.0),
            Token::Str("beta_i"),
            Token::F64(0.0),
            Token::Str("global_phase"),
            Token::F64(PI),
            Token::StructEnd,
        ],
    );
}

/// Test substitute parameters for SingleQubitGate
#[test]
fn test_singlequbitgate_substitute_parameters() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from("alpha_r"),
        CalculatorFloat::from("alpha_i"),
        CalculatorFloat::from("beta_r"),
        CalculatorFloat::from("beta_i"),
        CalculatorFloat::from("global_phase"),
    );
    assert_eq!(gate.alpha_r().clone(), CalculatorFloat::from("alpha_r"));
    assert_eq!(gate.alpha_i().clone(), CalculatorFloat::from("alpha_i"));
    assert_eq!(gate.beta_r().clone(), CalculatorFloat::from("beta_r"));
    assert_eq!(gate.beta_i().clone(), CalculatorFloat::from("beta_i"));
    assert_eq!(
        gate.global_phase().clone(),
        CalculatorFloat::from("global_phase")
    );
    assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("alpha_r", 0.0);
    substitution_dict.set_variable("alpha_i", 0.0);
    substitution_dict.set_variable("global_phase", PI);
    substitution_dict.set_variable("beta_r", 1.0);
    substitution_dict.set_variable("beta_i", -1.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.alpha_r().clone(), CalculatorFloat::from(0.0));
    assert_eq!(result.alpha_i().clone(), CalculatorFloat::from(0.0));
    assert_eq!(result.beta_r().clone(), CalculatorFloat::from(1.0));
    assert_eq!(result.beta_i().clone(), CalculatorFloat::from(-1.0));
    assert_eq!(result.global_phase().clone(), CalculatorFloat::from(PI));
}

///Test remap qubits for SingleQubitGate
#[test_case(0; "no-mapping")]
#[test_case(1; "1")]
#[test_case(100000; "100000")]
fn test_singlequbitgate_remap_qubits(newqubit: usize) {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    );
    let test_gate = SingleQubitGate::new(
        newqubit.clone(),
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    );
    // qubit mapping
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, newqubit.clone());
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(&result, &Ok(test_gate.clone()));

    // comparison of involved qubits (two variants, redundant on purpose)
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(newqubit.clone());
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    let result_unwrapped = result.unwrap();
    assert_eq!(
        result_unwrapped.involved_qubits(),
        test_gate.involved_qubits()
    );
    //testing involved_qubits() function
    assert_eq!(result_unwrapped.involved_qubits(), test_qubits);

    // does 'mapping back' work?
    qubit_mapping.remove(&0);
    qubit_mapping.insert(newqubit, 0);
    let result2 = result_unwrapped.remap_qubits(&qubit_mapping);
    assert_eq!(result2, Ok(gate));
}

/// Test debug for SingleQubitGate
#[test]
fn test_singlequbitgate_debug() {
    let gate = SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    );
    let name =  "SingleQubitGate { qubit: 0, alpha_r: Float(1.0), alpha_i: Float(0.0), beta_r: Float(0.0), beta_i: Float(0.0), global_phase: Float(3.141592653589793) }";
    assert_eq!(format!("{:?}", gate), name);
}

//
// unit tests for SingleQubitGate Operations
//

/// Test alpha, beta, global phase of single qubit gates with the unitary matrix
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(PI/3.0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(PI/3.0))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(PI/3.0))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(PI/3.0),
    CalculatorFloat::from(PI/2.0),
    CalculatorFloat::from(PI/4.0))); "Rotation")]
fn test_alpha_beta_singlequbitgates(gate: SingleQubitGateOperation) {
    let alpha_r = gate.alpha_r();
    let alpha_i = gate.alpha_i();
    let beta_r = gate.beta_r();
    let beta_i = gate.beta_i();
    let global_phase = gate.global_phase();
    let qubit = gate.qubit();
    let matrix = gate.unitary_matrix().unwrap();

    let singlequbitgate = SingleQubitGate::new(
        qubit.clone(),
        alpha_r,
        alpha_i,
        beta_r,
        beta_i,
        global_phase,
    );
    let test_matrix = singlequbitgate.unitary_matrix().unwrap();

    let epsilon = 1e-12;
    for i in 0..2 {
        assert!((matrix[[0, i]] - test_matrix[[0, i]]).norm() < epsilon);
        assert!((matrix[[1, i]] - test_matrix[[1, i]]).norm() < epsilon);
    }
}

/// Test RotateX,Y,Z rotate
#[test_case(0, CalculatorFloat::from(0); "rotate0")]
#[test_case(1, CalculatorFloat::from("theta"); "rotate1")]
fn test_rotatexyz_rotate(qubit: usize, theta: CalculatorFloat) {
    // Test RotateZ rotate
    let gate1 = RotateZ::new(qubit, theta.clone());
    let gate2 = RotateZ::new(qubit, CalculatorFloat::from(gate1.theta().clone()));
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);

    // Test RotateX rotate
    let gate1 = RotateX::new(qubit, theta.clone());
    let gate2 = RotateX::new(qubit, CalculatorFloat::from(gate1.theta().clone()));
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);

    // Test RotateY rotate
    let gate1 = RotateY::new(qubit, theta.clone());
    let gate2 = RotateY::new(qubit, CalculatorFloat::from(gate1.theta().clone()));
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);
}

/// Test theta() for PhaseShiftState gates
#[test_case(0, CalculatorFloat::from(0); "test0")]
#[test_case(1, CalculatorFloat::from("theta"); "test1")]
fn test_rotatexyz_phaseshiftstate(qubit: usize, theta: CalculatorFloat) {
    // Test PhaseShiftState0 rotate
    let gate1 = PhaseShiftState0::new(qubit, theta.clone());
    let gate2 = PhaseShiftState0::new(qubit, CalculatorFloat::from(gate1.theta().clone()));
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);

    // Test PhaseShiftState1 rotate
    let gate1 = PhaseShiftState1::new(qubit, theta.clone());
    let gate2 = PhaseShiftState1::new(qubit, CalculatorFloat::from(gate1.theta().clone()));
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);
}

/// Test rotate aroundsphericalaxis
#[test_case(0, CalculatorFloat::from(0), CalculatorFloat::from(0), CalculatorFloat::from(0); "rotate0")]
#[test_case(
    1,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi");
    "rotate1"
)]
fn test_rotatearoundsphericalaxis_rotate(
    qubit: usize,
    theta: CalculatorFloat,
    spherical_theta: CalculatorFloat,
    spherical_phi: CalculatorFloat,
) {
    // Test rotate AroundSphericalAxis
    let gate1 = RotateAroundSphericalAxis::new(
        qubit,
        theta.clone(),
        spherical_theta.clone(),
        spherical_phi.clone(),
    );
    let gate2 = RotateAroundSphericalAxis::new(
        gate1.qubit().clone(),
        CalculatorFloat::from(gate1.theta().clone()),
        spherical_theta.clone(),
        spherical_phi.clone(),
    );
    assert_eq!(gate1, gate2);
    let theta_p: &CalculatorFloat = gate1.theta();
    assert_eq!(theta_p, &theta);
    let spherical_theta_p: &CalculatorFloat = &gate1.spherical_theta();
    assert_eq!(spherical_theta_p, &spherical_theta);
    let spherical_phi_p: &CalculatorFloat = &gate1.spherical_phi();
    assert_eq!(spherical_phi_p, &spherical_phi);
}

/// Test 'qubit()' for SingleQubitGate Operations
#[test_case(0, SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "rotateX-0")]
#[test_case(1, SingleQubitGateOperation::from(RotateX::new(1, CalculatorFloat::from("theta"))); "rotateX-theta")]
#[test_case(0, SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "rotateY-0")]
#[test_case(1, SingleQubitGateOperation::from(RotateY::new(1, CalculatorFloat::from("theta"))); "rotateY-theta")]
#[test_case(0, SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "rotateZ-0")]
#[test_case(1, SingleQubitGateOperation::from(RotateZ::new(1, CalculatorFloat::from("theta"))); "rotateZ-theta")]
#[test_case(1, SingleQubitGateOperation::from(PauliX::new(1)); "Paulix")]
#[test_case(1, SingleQubitGateOperation::from(PauliY::new(1)); "Pauliy")]
#[test_case(1, SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(100, SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(100, SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(1, SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(1, SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(3, SingleQubitGateOperation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(0, SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi"))); "Rotation")]
#[test_case(0, SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(1, SingleQubitGateOperation::from(PhaseShiftState1::new(1, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
fn test_rotatexyz_operatesinglequbit(qubit: usize, gate: SingleQubitGateOperation) {
    let qubit_p: &usize = &gate.qubit();
    assert_eq!(qubit_p, &qubit);
}

/// Test 'clone()' for SingleQubitGate Operations
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from("theta"))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi"))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
fn test_rotatexyz_clone(gate1: SingleQubitGateOperation) {
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

/// Test 'hqslang()' for SingleQubitGate Operations
#[test_case("RotateX", SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case("RotateY", SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case("RotateZ", SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(
    "RotateAroundSphericalAxis",
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case("PauliX", SingleQubitGateOperation::from(PauliX::new(0)); "PauliX")]
#[test_case("PauliY", SingleQubitGateOperation::from(PauliY::new(0)); "PauliY")]
#[test_case("PauliZ", SingleQubitGateOperation::from(PauliZ::new(0)); "PauliZ")]
#[test_case("SqrtPauliX", SingleQubitGateOperation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case("InvSqrtPauliX", SingleQubitGateOperation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case("SGate", SingleQubitGateOperation::from(SGate::new(0)); "SGate")]
#[test_case("TGate", SingleQubitGateOperation::from(TGate::new(0)); "TGate")]
#[test_case("Hadamard", SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case("SingleQubitGate", SingleQubitGateOperation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    )); "SingleQubitGate")]
#[test_case("PhaseShiftState0", SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case("PhaseShiftState1", SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
fn test_singlequbitgateoperations_hqslang(name: &'static str, gate: SingleQubitGateOperation) {
    assert!(!gate.hqslang().is_empty());
    assert_eq!(gate.hqslang(), name);
}

/// Test (De-)serialization of RotateX,Y,Z
#[cfg(feature = "serialize")]
#[test_case("RotateX", SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[cfg(feature = "serialize")]
#[test_case("RotateY", SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[cfg(feature = "serialize")]
#[test_case("RotateZ", SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
fn ser_de_rotate_xyz(name: &'static str, gate: SingleQubitGateOperation) {
    assert_tokens(
        &gate.readable(),
        &[
            Token::NewtypeVariant {
                name: "SingleQubitGateOperation",
                variant: name,
            },
            Token::Struct { name: name, len: 2 },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("theta"),
            Token::F64(0.0),
            Token::StructEnd,
        ],
    );
}

/// Test (De-)serialization of gate RotateAroundSphericalAxis
#[cfg(feature = "serialize")]
#[test_case(
    "RotateAroundSphericalAxis",
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
fn ser_de_rotate_aroundsphericalaxis(name: &'static str, gate: SingleQubitGateOperation) {
    assert_tokens(
        &gate.readable(),
        &[
            Token::NewtypeVariant {
                name: "SingleQubitGateOperation",
                variant: name,
            },
            Token::Struct { name: name, len: 4 },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("theta"),
            Token::F64(0.0),
            Token::Str("spherical_theta"),
            Token::F64(0.0),
            Token::Str("spherical_phi"),
            Token::F64(0.0),
            Token::StructEnd,
        ],
    );
}

/// Test (De-)serialization of single qubit gates
#[cfg(feature = "serialize")]
#[test_case("PauliX", SingleQubitGateOperation::from(PauliX::new(0)); "PauliX")]
#[cfg(feature = "serialize")]
#[test_case("PauliY", SingleQubitGateOperation::from(PauliY::new(0)); "PauliY")]
#[cfg(feature = "serialize")]
#[test_case("PauliZ", SingleQubitGateOperation::from(PauliZ::new(0)); "PauliZ")]
#[cfg(feature = "serialize")]
#[test_case("SqrtPauliX", SingleQubitGateOperation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[cfg(feature = "serialize")]
#[test_case("InvSqrtPauliX", SingleQubitGateOperation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[cfg(feature = "serialize")]
#[test_case("SGate", SingleQubitGateOperation::from(SGate::new(0)); "SGate")]
#[cfg(feature = "serialize")]
#[test_case("TGate", SingleQubitGateOperation::from(TGate::new(0)); "TGate")]
#[cfg(feature = "serialize")]
#[test_case("Hadamard", SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
fn ser_de_singlequbitgates_others(name: &'static str, gate: SingleQubitGateOperation) {
    assert_tokens(
        &gate.readable(),
        &[
            Token::NewtypeVariant {
                name: "SingleQubitGateOperation",
                variant: name,
            },
            Token::Struct { name: name, len: 1 },
            Token::Str("qubit"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

/// Test RotateZ alpha, beta, global phase
#[test_case(0.0, 1.0, 0.0; "theta = 0")]
#[test_case(PI/2.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0 * (-1.0); "theta = pi/2")]
#[test_case(PI, 0.0, -1.0; "theta = pi")]
fn test_rotatez_abp(theta: f64, cos: f64, sin: f64) {
    let gate: RotateZ = RotateZ::new(0, CalculatorFloat::from(theta));
    // verify alpha, beta, global_phase as per definition
    assert_eq!((gate.theta().clone() / 2.0).cos(), gate.alpha_r());
    assert_eq!((gate.theta().clone() / 2.0).sin() * (-1.0), gate.alpha_i());
    assert_eq!(CalculatorFloat::from(0), gate.beta_r());
    assert_eq!(CalculatorFloat::from(0), gate.beta_i());
    assert_eq!(CalculatorFloat::from(0), gate.global_phase());
    // verify expected function values dependent on theta
    assert!((f64::try_from(gate.alpha_r()).unwrap() - cos).abs() < EPSILON);
    assert!((f64::try_from(gate.alpha_i()).unwrap() - sin).abs() < EPSILON);
}

/// Test RotateX alpha, beta, global phase
#[test_case(0.0, 1.0, 0.0; "theta = 0")]
#[test_case(PI/2.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0 * (-1.0); "theta = pi/2")]
#[test_case(PI, 0.0, -1.0; "theta = pi")]
fn test_rotatex_abp(theta: f64, cos: f64, sin: f64) {
    let gate: RotateX = RotateX::new(0, CalculatorFloat::from(theta));
    // verify alpha, beta, global_phase as per definition
    assert_eq!((gate.theta().clone() / 2.0).cos(), gate.alpha_r());
    assert_eq!(CalculatorFloat::from(0), gate.alpha_i());
    assert_eq!(CalculatorFloat::from(0), gate.beta_r());
    assert_eq!((gate.theta().clone() / 2.0).sin() * (-1.0), gate.beta_i());
    assert_eq!(CalculatorFloat::from(0), gate.global_phase());
    // verify expected function values dependent on theta
    assert!((f64::try_from(gate.alpha_r()).unwrap() - cos).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_i()).unwrap() - sin).abs() < EPSILON);
}

/// Test RotateY alpha, beta, global phase
#[test_case(0.0, 1.0, 0.0; "theta = 0")]
#[test_case(PI/2.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0; "theta = pi/2")]
#[test_case(PI, 0.0, 1.0; "theta = pi")]
fn test_rotatey_abp(theta: f64, cos: f64, sin: f64) {
    let gate: RotateY = RotateY::new(0, CalculatorFloat::from(theta));
    // verify alpha, beta, global_phase as per definition
    assert_eq!((gate.theta().clone() / 2.0).cos(), gate.alpha_r());
    assert_eq!(CalculatorFloat::from(0), gate.alpha_i());
    assert_eq!((gate.theta().clone() / 2.0).sin(), gate.beta_r());
    assert_eq!(CalculatorFloat::from(0), gate.beta_i());
    assert_eq!(CalculatorFloat::from(0), gate.global_phase());
    // verify expected function values dependent on theta
    assert!((f64::try_from(gate.alpha_r()).unwrap() - cos).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_r()).unwrap() - sin).abs() < EPSILON);
}

/// Test alpha, beta, global phase of RotateAroundSphericalAxis
#[test_case(
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    1.0, 0.0, 0.0, 0.0, 0.0; "theta = 0")]
#[test_case(
    CalculatorFloat::from(PI),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    0.0, -1.0, 0.0, 0.0, 0.0; "theta = PI")]
#[test_case(
    CalculatorFloat::from(PI),
    CalculatorFloat::from(PI / 2.0),
    CalculatorFloat::from(0),
    0.0, 0.0, 0.0, -1.0, 0.0; "theta_sp = PI/2")]
#[test_case(
    CalculatorFloat::from(PI),
    CalculatorFloat::from(PI / 2.0),
    CalculatorFloat::from(PI / 2.0),
    0.0, 0.0, 1.0, 0.0, 0.0; "phi = PI/2")]
fn test_rotatearoundsphericalaxis_abp(
    theta: CalculatorFloat,
    spherical_theta: CalculatorFloat,
    spherical_phi: CalculatorFloat,
    alpha_r: f64,
    alpha_i: f64,
    beta_r: f64,
    beta_i: f64,
    global_phase: f64,
) {
    let gate = RotateAroundSphericalAxis::new(0, theta, spherical_theta, spherical_phi);
    // verify alpha, beta, global_phase as per definition
    assert!((f64::try_from(gate.alpha_r()).unwrap() - alpha_r).abs() < EPSILON);
    assert!((f64::try_from(gate.alpha_i()).unwrap() - alpha_i).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_r()).unwrap() - beta_r).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_i()).unwrap() - beta_i).abs() < EPSILON);
    assert_eq!(CalculatorFloat::from(global_phase), gate.global_phase());
}

/// Test alpha, beta, global phase of SingleQubitGate Operations
#[test_case(
    0.0, 0.0, 0.0, -1.0, PI / 2.0,
    SingleQubitGateOperation::from(PauliX::new(0)); "PauliX")]
#[test_case(
    0.0, 0.0, 1.0, 0.0, PI / 2.0,
    SingleQubitGateOperation::from(PauliY::new(0)); "PauliY")]
#[test_case(
    0.0, -1.0, 0.0, 0.0, PI / 2.0,
    SingleQubitGateOperation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(
    (PI / 4.0).cos(), 0.0, 0.0, (-1.0) * (PI / 4.0).cos(), 0.0,
    SingleQubitGateOperation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(
    (PI / 4.0).cos(), 0.0, 0.0, (PI / 4.0).cos(), 0.0,
    SingleQubitGateOperation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(
    (PI / 8.0).cos(), (-1.0) * (PI / 8.0).sin(), 0.0, 0.0, PI / 8.0,
    SingleQubitGateOperation::from(TGate::new(0)); "TGate")]
#[test_case(
    1.0 / (2.0_f64).sqrt(), (-1.0) / (2.0_f64).sqrt(), 0.0, 0.0, PI / 4.0,
    SingleQubitGateOperation::from(SGate::new(0)); "SGate")]
#[test_case(
    0.0, (-1.0) / (2.0_f64).sqrt(), 0.0,(-1.0) / (2.0_f64).sqrt(), PI / 2.0,
    SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
fn test_singlequbitgates_abp(
    alpha_r: f64,
    alpha_i: f64,
    beta_r: f64,
    beta_i: f64,
    global_phase: f64,
    gate: SingleQubitGateOperation,
) {
    assert!((f64::try_from(gate.alpha_r()).unwrap() - alpha_r).abs() < EPSILON);
    assert!((f64::try_from(gate.alpha_i()).unwrap() - alpha_i).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_r()).unwrap() - beta_r).abs() < EPSILON);
    assert!((f64::try_from(gate.beta_i()).unwrap() - beta_i).abs() < EPSILON);
    assert_eq!(CalculatorFloat::from(global_phase), gate.global_phase());
}

/// Test is_parametrized for SingleQubitGate Operations
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
fn test_is_parametrized_false(gate: SingleQubitGateOperation) {
    let bool_parameter = gate.is_parametrized();
    assert!(!bool_parameter);
}

/// Test unitarity of the matrix for SingleQubitGate Operations
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
fn test_singlequbitgates_unitarity(gate: SingleQubitGateOperation) {
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    let result_matrix: Array2<Complex64> = result.unwrap();

    // check unitarity with nalgebra
    // convert ndarray into nalgebra matrix
    let result_matrix2 = na::Matrix2::new(
        result_matrix[[0, 0]],
        result_matrix[[0, 1]],
        result_matrix[[1, 0]],
        result_matrix[[1, 1]],
    );
    // calculate matrix product A*A_dagger
    let product = result_matrix2 * result_matrix2.adjoint(); // associated function 'self.conjugate_transpose()'  renamed 'self.adjoint()'.

    // convert complex matrix product into real matrix by taking the absolute value of the complex number, which should be sufficient if the matrix is unitary.
    let matrix_norm: na::Matrix2<f64> = na::Matrix2::new(
        product[0].norm_sqr(),
        product[1].norm_sqr(),
        product[2].norm_sqr(),
        product[3].norm_sqr(),
    );
    let epsilon = 1e-12;
    assert!(matrix_norm.is_identity(epsilon));
}

/// Test RotateX substitute parameters
#[test]
fn test_rotatex_substitute_parameters() {
    //fn substitute_parameters(&self, calculator: &mut Calculator) -> Result<Self, RoqoqoError>;
    //fn is_parametrized(&self) -> bool;
    let gate: RotateX = RotateX::new(0, CalculatorFloat::from("theta"));
    assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.theta().clone(), CalculatorFloat::from(0.0));
}

/// Test substitute parameters function for SingleQubitGate Operations where it has no effect
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
#[test_case(SingleQubitGateOperation::from(SingleQubitGate::new(
    0,
    CalculatorFloat::from(1.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
    CalculatorFloat::from(0.0),
)); "singlequbitgate")]
fn test_ineffective_substitute_parameters(gate: SingleQubitGateOperation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert_eq!(result, gate.clone());
}

/// Test RotateY substitute parameters
#[test]
fn test_rotatey_substitute_parameters() {
    let gate: RotateY = RotateY::new(0, CalculatorFloat::from("theta"));
    assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.theta().clone(), CalculatorFloat::from(0.0));
}

/// Test RotateZ substitute parameters
#[test]
fn test_rotatez_substitute_parameters() {
    let gate: RotateZ = RotateZ::new(0, CalculatorFloat::from("theta"));
    assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.theta().clone(), CalculatorFloat::from(0.0));
}

/// Test substitute_parameters for PhaseShiftState0
// #[test]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from("theta"))),
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "state0"
)]
fn test_phaseshiftstate0_substitute_parameters(
    gate: SingleQubitGateOperation,
    testgate: SingleQubitGateOperation,
) {
    // let gate: PhaseShiftState0 = PhaseShiftState0::new(0, CalculatorFloat::from("theta"));
    // assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    // assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    // assert!(!result.is_parametrized());
    assert_eq!(result, testgate);
}

/// Test substitute_parameters for PhaseShiftState1
#[test]
fn test_phaseshiftstate1_substitute_parameters() {
    let gate: PhaseShiftState1 = PhaseShiftState1::new(0, CalculatorFloat::from("theta"));
    assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    assert!(gate.is_parametrized());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.theta().clone(), CalculatorFloat::from(0.0));
}

/// Test substitute parameters for RotateAroundSphericalAxis
#[test]
fn test_rotatearoundsphericalaxis_substitute_parameters() {
    let gate = RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("spherical_theta"),
        CalculatorFloat::from("spherical_phi"),
    );
    assert!(gate.is_parametrized());
    assert_eq!(gate.theta().clone(), CalculatorFloat::from("theta"));
    assert_eq!(
        gate.spherical_theta().clone(),
        CalculatorFloat::from("spherical_theta"),
    );
    assert_eq!(
        gate.spherical_phi().clone(),
        CalculatorFloat::from("spherical_phi"),
    );
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    substitution_dict.set_variable("spherical_theta", PI);
    substitution_dict.set_variable("spherical_phi", PI / 2.0);
    let result = gate.substitute_parameters(&mut substitution_dict).unwrap();
    assert!(!result.is_parametrized());
    assert_eq!(result.theta().clone(), CalculatorFloat::from(0.0));
    assert_eq!(result.spherical_theta().clone(), CalculatorFloat::from(PI));
    assert_eq!(
        result.spherical_phi().clone(),
        CalculatorFloat::from(PI / 2.0)
    );
}

/// Test remap qubits for SingleQubitGate Operations
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateX::new(2, CalculatorFloat::from(0))),
    2; "RotateX_0-2")]
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    0; "RotateX_nomapping")]
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateX::new(100, CalculatorFloat::from(0))),
    100; "RotateX_0-100")]
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateY::new(2, CalculatorFloat::from(0))),
    2; "RotateY_0-2")]
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    0; "RotateY_nomapping")]
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateY::new(100, CalculatorFloat::from(0))),
    100; "RotateY_0-100")]
#[test_case(
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateZ::new(2, CalculatorFloat::from(0))),
    2; "RotateZ_0-2")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(PhaseShiftState0::new(2, CalculatorFloat::from(0))),
    2; "PhaseShiftState0")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(PhaseShiftState1::new(2, CalculatorFloat::from(0))),
    2; "PhaseShiftState1")]
#[test_case(
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    0; "RotateZ_nomapping")]
#[test_case(
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateZ::new(100, CalculatorFloat::from(0))),
    100; "RotateZ_0-100")]
#[test_case(
    SingleQubitGateOperation::from(PauliX::new(0)),
    SingleQubitGateOperation::from(PauliX::new(5)),
    5; "PauliX_0-5")]
#[test_case(
    SingleQubitGateOperation::from(PauliY::new(0)),
    SingleQubitGateOperation::from(PauliY::new(5)),
    5; "PauliY_0-5")]
#[test_case(
    SingleQubitGateOperation::from(PauliZ::new(0)),
    SingleQubitGateOperation::from(PauliZ::new(5)),
    5; "PauliZ_0-5")]
#[test_case(
    SingleQubitGateOperation::from(SqrtPauliX::new(0)),
    SingleQubitGateOperation::from(SqrtPauliX::new(1)),
    1; "SqrtPauliX_0-1")]
#[test_case(
    SingleQubitGateOperation::from(InvSqrtPauliX::new(0)),
    SingleQubitGateOperation::from(InvSqrtPauliX::new(1)),
    1; "InvSqrtPauliX_0-1")]
#[test_case(
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            3,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    3; "Rotation")]
#[test_case(
    SingleQubitGateOperation::from(SGate::new(0)),
    SingleQubitGateOperation::from(SGate::new(1)),
    1; "SGate_0-1")]
#[test_case(
    SingleQubitGateOperation::from(TGate::new(0)),
    SingleQubitGateOperation::from(TGate::new(1)),
    1; "TGate_0-1")]
#[test_case(
    SingleQubitGateOperation::from(Hadamard::new(0)),
    SingleQubitGateOperation::from(Hadamard::new(1)),
    1; "Hadamard")]
fn test_singlequbitgates_remap_qubits(
    operation: SingleQubitGateOperation,
    test_operation: SingleQubitGateOperation,
    newqubit: usize,
) {
    let gate = operation.clone();
    // qubit mapping
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, newqubit.clone());
    let result_wrapped = gate.remap_qubits(&qubit_mapping);

    // comparison of Result
    assert_eq!(&result_wrapped, &Ok(test_operation.clone()));

    // comparison of qubits from the unwrapped Result
    let result = result_wrapped.unwrap();
    assert_eq!(result.qubit(), &newqubit.clone());

    // comparison of involved qubits (two variants, redundant on purpose)
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(newqubit.clone());
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    assert_eq!(
        result.involved_qubits(),
        test_operation.clone().involved_qubits()
    );
    //testing involved_qubits() function
    assert_eq!(result.involved_qubits(), test_qubits);

    // does 'mapping back' work?
    qubit_mapping.remove(&0);
    qubit_mapping.insert(newqubit, 0);
    let result2 = result.remap_qubits(&qubit_mapping);
    assert_eq!(result2, Ok(operation.clone()));
}

/// Test error case of remap_qubits() function for SingleQubitGateOperations
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(SingleQubitGateOperation::from(PauliX::new(0)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(0)); "PauliY")]
#[test_case( SingleQubitGateOperation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(0)); "SGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(0)); "TGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    )); "SingleQubitGate")]
fn remap_qubits_error(gate: SingleQubitGateOperation) {
    let qubit_mapping: HashMap<usize, usize> = HashMap::new();
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 0 }));
}

/// Test debug for SingleQubitGate Operations
#[test_case(
    "RotateX(RotateX { qubit: 0, theta: Float(0.0) })",
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0)));
    "RotateX")]
#[test_case(
    "RotateY(RotateY { qubit: 0, theta: Float(0.0) })",
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0)));
    "RotateY")]
#[test_case(
    "RotateZ(RotateZ { qubit: 0, theta: Float(0.0) })",
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0)));
    "RotateZ")]
#[test_case(
    "PauliX(PauliX { qubit: 0 })",
    SingleQubitGateOperation::from(PauliX::new(0));
    "PauliX")]
#[test_case(
    "PauliY(PauliY { qubit: 0 })",
    SingleQubitGateOperation::from(PauliY::new(0));
    "PauliY")]
#[test_case(
    "PauliZ(PauliZ { qubit: 0 })",
    SingleQubitGateOperation::from(PauliZ::new(0));
    "PauliZ")]
#[test_case(
    "SqrtPauliX(SqrtPauliX { qubit: 0 })",
    SingleQubitGateOperation::from(SqrtPauliX::new(0));
    "SqrtPauliX")]
#[test_case(
    "InvSqrtPauliX(InvSqrtPauliX { qubit: 0 })",
    SingleQubitGateOperation::from(InvSqrtPauliX::new(0));
    "InvSqrtPauliX")]
#[test_case(
    "SGate(SGate { qubit: 0 })",
    SingleQubitGateOperation::from(SGate::new(0));
    "SGate")]
#[test_case(
    "TGate(TGate { qubit: 0 })",
    SingleQubitGateOperation::from(TGate::new(0));
    "TGate")]
#[test_case(
    "Hadamard(Hadamard { qubit: 0 })",
    SingleQubitGateOperation::from(Hadamard::new(0));
    "Hadamard")]
#[test_case(
    "RotateAroundSphericalAxis(RotateAroundSphericalAxis { qubit: 0, theta: Float(0.0), spherical_theta: Float(0.0), spherical_phi: Float(0.0) })",
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(
    "PhaseShiftState0(PhaseShiftState0 { qubit: 0, theta: Float(0.0) })",
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(
    "PhaseShiftState1(PhaseShiftState1 { qubit: 0, theta: Float(0.0) })",
    SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))); "PhaseShiftState1")]
fn test_singlequbitgates_debug(name: &'static str, gate: SingleQubitGateOperation) {
    assert_eq!(format!("{:?}", gate), name);
}

/// Test PartialEq for SingleQubitGate Operations
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(1, CalculatorFloat::from(1))),
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0)));
    "RotateX")]
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(1, CalculatorFloat::from(1))),
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0)));
    "RotateY")]
#[test_case(
    SingleQubitGateOperation::from(RotateZ::new(1, CalculatorFloat::from(1))),
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0)));
    "RotateZ")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState1::new(1, CalculatorFloat::from(1))),
    SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0)));
    "PhaseShiftState1")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState0::new(1, CalculatorFloat::from(1))),
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0)));
    "PhaseShiftState0")]
#[test_case(
    SingleQubitGateOperation::from(PauliX::new(1)),
    SingleQubitGateOperation::from(PauliX::new(0));
    "PauliX")]
#[test_case(
    SingleQubitGateOperation::from(PauliY::new(1)),
    SingleQubitGateOperation::from(PauliY::new(0));
    "PauliY")]
#[test_case(
    SingleQubitGateOperation::from(PauliZ::new(1)),
    SingleQubitGateOperation::from(PauliZ::new(0));
    "PauliZ")]
#[test_case(
    SingleQubitGateOperation::from(SqrtPauliX::new(1)),
    SingleQubitGateOperation::from(SqrtPauliX::new(0));
    "SqrtPauliX")]
#[test_case(
    SingleQubitGateOperation::from(InvSqrtPauliX::new(1)),
    SingleQubitGateOperation::from(InvSqrtPauliX::new(0));
    "InvSqrtPauliX")]
#[test_case(
    SingleQubitGateOperation::from(SGate::new(1)),
    SingleQubitGateOperation::from(SGate::new(0));
    "SGate")]
#[test_case(
    SingleQubitGateOperation::from(TGate::new(1)),
    SingleQubitGateOperation::from(TGate::new(0));
    "TGate")]
#[test_case(
    SingleQubitGateOperation::from(Hadamard::new(1)),
    SingleQubitGateOperation::from(Hadamard::new(0));
    "Hadamard")]
#[test_case(
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            1,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(
    SingleQubitGateOperation::from(
        SingleQubitGate::new(
            0,
            CalculatorFloat::from(1.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(PI),
        )
    ),
    SingleQubitGateOperation::from(
        SingleQubitGate::new(
            1,
            CalculatorFloat::from(1.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(0.0),
            CalculatorFloat::from(PI),
        )
    ); "SingleQubitGate")]
fn test_singlequbitgates_partialeq(
    gate1: SingleQubitGateOperation,
    gate2: SingleQubitGateOperation,
) {
    assert!(gate1.clone() == gate1);
    assert!(gate1 == gate1.clone());
    assert!(gate2 != gate1);
    assert!(gate1 != gate2);
}

/// Test SingleQubitGate multiplication for RotateXYZ
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0)));
    "RotateXYZ")]
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateY::new(1, CalculatorFloat::from(0))),
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0)));
    "RotateXYZ-err")]
fn test_rotatexyz_multiplication(
    gate1: SingleQubitGateOperation,
    gate2: SingleQubitGateOperation,
    gate3: SingleQubitGateOperation,
) {
    let result = gate1.mul(&gate2);

    if gate1.qubit() != gate2.qubit() {
        // single qubit gates are supposed to apply on the same qubit
        assert!(result.is_err());
    } else {
        let multiplied = result.unwrap();
        assert_eq!(multiplied.alpha_r(), gate3.alpha_r());
        assert_eq!(multiplied.alpha_i(), gate3.alpha_i());
        assert_eq!(multiplied.beta_r(), gate3.beta_r());
        assert_eq!(multiplied.beta_i(), gate3.beta_i());
        assert_eq!(multiplied.global_phase(), gate3.global_phase());
    }
}

/// Test SingleQubitGate multiplication for Hadamard gate
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(PI / 2.0))),
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(PI))),
    SingleQubitGateOperation::from(Hadamard::new(0));
    "Hadamard")]
fn test_hadamard_multiplication(
    gate1: SingleQubitGateOperation,
    gate2: SingleQubitGateOperation,
    gate3: SingleQubitGateOperation,
) {
    let result = gate1.mul(&gate2);
    assert!(result.is_ok());
    // verify that Ry(pi/2) * Rz(pi) = i*H
    let multiplied = result.unwrap();
    let epsilon = 1e-12;
    assert!(
        (f64::try_from(multiplied.alpha_r()).unwrap() - f64::try_from(gate3.alpha_r()).unwrap())
            .abs()
            < epsilon
    );
    assert!(
        (f64::try_from(multiplied.alpha_i()).unwrap() - f64::try_from(gate3.alpha_i()).unwrap())
            .abs()
            < epsilon
    );
    assert!(
        (f64::try_from(multiplied.beta_r()).unwrap() - f64::try_from(gate3.beta_r()).unwrap())
            .abs()
            < epsilon
    );
    assert!(
        (f64::try_from(multiplied.beta_i()).unwrap() - f64::try_from(gate3.beta_i()).unwrap())
            .abs()
            < epsilon
    );
    // if f64::try_from(gate3.global_phase()).unwrap() + PI / 2.0 == 2.0 * PI {
    //     assert_eq!(multiplied.global_phase(), CalculatorFloat::from(0.0));
    // } else {
    //     assert_eq!(multiplied.global_phase(), gate3.global_phase() + PI / 2.0);
    // }
}

/// Test SingleQubitGate multiplication for Pauli matrices
#[test_case(
    SingleQubitGateOperation::from(PauliX::new(0)),
    SingleQubitGateOperation::from(PauliY::new(0)),
    SingleQubitGateOperation::from(PauliZ::new(0));
    "permutation1")]
#[test_case(
    SingleQubitGateOperation::from(PauliY::new(2)),
    SingleQubitGateOperation::from(PauliZ::new(2)),
    SingleQubitGateOperation::from(PauliX::new(0));
    "permutation2")]
#[test_case(
    SingleQubitGateOperation::from(PauliZ::new(0)),
    SingleQubitGateOperation::from(PauliX::new(0)),
    SingleQubitGateOperation::from(PauliY::new(2));
    "permutation3")]
#[test_case(
    SingleQubitGateOperation::from(PauliX::new(1)),
    SingleQubitGateOperation::from(PauliY::new(0)),
    SingleQubitGateOperation::from(PauliZ::new(2));
    "error")]
fn test_pualixyz_multiplication(
    gate1: SingleQubitGateOperation,
    gate2: SingleQubitGateOperation,
    gate3: SingleQubitGateOperation,
) {
    let result = gate1.mul(&gate2);

    // verify that Pauli_k + Pauli_l = i * Pauli_m
    if gate1.qubit() != gate2.qubit() {
        // single qubit gates are supposed to apply on the same qubit
        assert!(result.is_err());
    } else {
        let multiplied = result.unwrap();
        assert_eq!(multiplied.alpha_r(), gate3.alpha_r());
        assert_eq!(multiplied.alpha_i(), gate3.alpha_i());
        assert_eq!(multiplied.beta_r(), gate3.beta_r());
        assert_eq!(multiplied.beta_i(), gate3.beta_i());
        assert_eq!(multiplied.global_phase(), gate3.global_phase() + PI / 2.0);
    }
}

/// Test powerfc function for rotate gates - here representative RotateX
#[test_case(CalculatorFloat::from(0); "0")]
#[test_case(CalculatorFloat::from(PI / 4.0); "pi/4")]
fn test_rotatex_powercf_2(theta: CalculatorFloat) {
    let gate = RotateX::new(0, theta);
    let power_gate = gate.powercf(CalculatorFloat::from(2.0));
    let test_gate = gate.mul(&gate).unwrap();
    assert!(
        (f64::try_from(power_gate.alpha_r()).unwrap()
            - f64::try_from(test_gate.alpha_r()).unwrap())
        .abs()
            < EPSILON
    );
    assert!(
        (f64::try_from(power_gate.alpha_i()).unwrap()
            - f64::try_from(test_gate.alpha_i()).unwrap())
        .abs()
            < EPSILON
    );
    assert!(
        (f64::try_from(power_gate.beta_r()).unwrap() - f64::try_from(test_gate.beta_r()).unwrap())
            .abs()
            < EPSILON
    );
    assert!(
        (f64::try_from(power_gate.beta_i()).unwrap() - f64::try_from(test_gate.beta_i()).unwrap())
            .abs()
            < EPSILON
    );
    assert!(
        (f64::try_from(power_gate.global_phase()).unwrap()
            - f64::try_from(test_gate.global_phase()).unwrap())
        .abs()
            < EPSILON
    );
}

/// Test powerfc function for RotateX with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(2.0); "power_2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0 / 2.0); "power_1/2")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(1.0); "power_1")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(0.0); "power_0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from(-2.0); "power_-2.0")]
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_rotatex_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let gate = RotateX::new(0, theta);
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = RotateX::new(0, test_theta);
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test powerfc function for RotateY with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_rotatey_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let gate = RotateY::new(0, theta);
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = RotateY::new(0, test_theta);
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test powerfc function for RotateZ with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_rotatez_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let gate = RotateZ::new(0, theta);
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = RotateZ::new(0, test_theta);
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test powerfc function for PhaseShiftState0 with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_phaseshiftstate0_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let gate = PhaseShiftState0::new(0, theta);
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = PhaseShiftState0::new(0, test_theta);
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test powerfc function for PhaseShiftState1 with symbolic parameters
#[test_case(CalculatorFloat::from("theta"), CalculatorFloat::from("power"); "power_symbolic")]
fn test_phaseshiftstate1_powercf(theta: CalculatorFloat, power: CalculatorFloat) {
    let gate = PhaseShiftState1::new(0, theta);
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = PhaseShiftState1::new(0, test_theta);
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test powerfc function for RotateAroundSphericalAxis
#[test_case(
    0,
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0),
    CalculatorFloat::from(0);
    "rotate0"
)]
#[test_case(
    1,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi"), 
    CalculatorFloat::from("power");
    "rotate1"
)]
#[test_case(
    1,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi"), 
    CalculatorFloat::from(2.0);
    "rotate2"
)]
fn test_rotatearoundsphericalaxis_powerfc(
    qubit: usize,
    theta: CalculatorFloat,
    spherical_theta: CalculatorFloat,
    spherical_phi: CalculatorFloat,
    power: CalculatorFloat,
) {
    // Test rotate AroundSphericalAxis
    let gate = RotateAroundSphericalAxis::new(
        qubit.clone(),
        theta.clone(),
        spherical_theta.clone(),
        spherical_phi.clone(),
    );
    let power_gate = gate.powercf(power.clone());
    let test_theta = power * gate.theta().clone();
    let test_gate = RotateAroundSphericalAxis::new(
        qubit.clone(),
        test_theta,
        spherical_theta.clone(),
        spherical_phi.clone(),
    );
    assert_eq!(power_gate.alpha_r(), test_gate.alpha_r());
    assert_eq!(power_gate.alpha_i(), test_gate.alpha_i());
    assert_eq!(power_gate.beta_r(), test_gate.beta_r());
    assert_eq!(power_gate.beta_i(), test_gate.beta_i());
    assert_eq!(power_gate.global_phase(), test_gate.global_phase());
}

/// Test tags() function for SingleQubitGateOperations
#[test_case(
    SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateX",
        ];
    "RotateX")]
#[test_case(
    SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateY",
        ];
    "RotateY")]
#[test_case(
    SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateZ",
        ];
    "RotateZ")]
#[test_case(
    SingleQubitGateOperation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateAroundSphericalAxis",
        ];
    "RotateAroundSphericalAxis")]
#[test_case(
    SingleQubitGateOperation::from(TGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "TGate",
        ];
    "TGate")]
#[test_case(
    SingleQubitGateOperation::from(SGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SGate",
        ];
    "SGate")]
#[test_case(
    SingleQubitGateOperation::from(PauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliX",
        ];
    "PauliX")]
#[test_case(
    SingleQubitGateOperation::from(PauliY::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliY",
        ];
    "PauliY")]
#[test_case(
    SingleQubitGateOperation::from(PauliZ::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliZ",
        ];
    "PauliZ")]
#[test_case(
    SingleQubitGateOperation::from(SqrtPauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SqrtPauliX",
        ];
    "SqrtPauliX")]
#[test_case(
    SingleQubitGateOperation::from(InvSqrtPauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvSqrtPauliX",
        ];
    "InvSqrtPauliX")]
#[test_case(
    SingleQubitGateOperation::from(Hadamard::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Hadamard",
        ];
    "Hadamard")]
#[test_case(SingleQubitGateOperation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    )),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SingleQubitGate",
        ];
    "SingleQubitGate")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "PhaseShiftState1",
        ];
    "PhaseShiftState1")]
#[test_case(
    SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "PhaseShiftState0",
        ];
    "PhaseShiftState0")]
pub fn test_tags(gate: SingleQubitGateOperation, tags: Vec<&str>) {
    for i in 0..tags.len() {
        assert_eq!(gate.tags()[i], tags[i]);
    }
}
