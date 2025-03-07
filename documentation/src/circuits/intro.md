# Quantum circuit

A quantum circuit refers to a linear sequence of operations that can be executed on a quantum computer. Like most quantum computing toolkits, qoqo/roqoqo provides a `Circuit` object and a set of `Operations` that can be added to a `Circuit`.

qoqo/roqoqo distinguishes between:

* Definitions: Operations that declare (and initialize) classical register values ([see also here](readout.md))
* Gate Operations: Unitary operations that can be executed on every unitary quantum computer (but might need to be decomposed into a sequence of native operations) ([see also here](unitary.md))
* Pragma operations that provide additional functionality to a quantum program, and are not generally available on all universal quantum computers (see [pragma operations](pragma.md) and [noise operations](noise.md) )

In order to create a useful result, a Circuit in qoqo/roqoqo must contain:

* A definition of a classical register for readout
* Operations to change the state of the quantum computer, for example `RotateZ` or `CNOT` gate operations.
* A measurement to return classical information based on the state of the quantum computer.

With qoqo, a `Circuit` can be constructed like this:

```python
from qoqo import Circuit
from qoqo import operations as ops
# create a new circuit
circuit = Circuit()
# Define the readout for two qubits 
circuit += ops.DefinitionBit(name="ro", length=2, is_output=True)
# Rotation around Z axis by pi/2 on qubit 0
circuit += ops.RotateZ(qubit=0, theta=1.57)
# Entangling qubits 0 and 1 with CNOT gate
circuit += ops.CNOT(control=0, target=1)
# Measuring the qubits
circuit += ops.MeasureQubit(qubit=0, readout="ro", readout_index=0)
circuit += ops.MeasureQubit(qubit=1, readout="ro", readout_index=1)
```

And with roqoqo, like this:

```rust
use roqoqo::{Circuit, operations::*};

// Create a new _modifiable_ circuit
let mut circuit = Circuit::new();
// Define the readout for two qubits 
circuit += DefinitionBit::new("ro".to_string(), 2, true);
// Apply rotation around Z axis by pi/2 on qubit 0
circuit += RotateZ::new(0, 1.57.into());
// Establish entanglement between qubits 0 and 1
circuit += CNOT::new(0, 1);
// Measuring the qubits
circuit += MeasureQubit::new(0, "ro".to_string(), 0);
circuit += MeasureQubit::new(1, "ro".to_string(), 1);
```

A circuit created with qoqo can also be visualized. The user can do this by installing the package `qollage` in python or the crate `roqollage` in rust.
It can be installed the same way as qoqo/roqoqo:

To install the package in a python environment run the following command
```bash
pip install qollage
```

or add this line to the Cargo.toml of a rust project

```TOML
qollage = "0.5" # Change this to the latest version to ensure compatibility of the latest qoqo operations.
```

In python the following code will output an image of the user's circuit using IPython’s display method.

```python
from qollage import draw_circuit

# draw the circuit
draw_circuit(circuit)
```
The generated image of the previous circuit should look like this:

<img src="./images/circuit_example.png" alt="circuit" width="60%">

In Rust, the image can be retrieved with the following function:

```rust
let image =
        roqollage::circuit_to_image(&circuit, None, roqollage::RenderPragmas::All, None, None)
            .expect("Failed to create an image of the circuit.");
```

For more information about this tool, please see the full documentation of [qollage](https://github.com/HQSquantumsimulations/qollage).

For details on the **available methods** of a `Circuit` please refer to the **API documentation** of [roqoqo](https://docs.rs/roqoqo/latest/roqoqo/struct.Circuit.html) and [qoqo](https://hqsquantumsimulations.github.io/qoqo/generated/qoqo.html#qoqo.Circuit).
