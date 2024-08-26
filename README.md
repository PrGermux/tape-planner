# Tape Planner

Tape Planner is a desktop application designed to assist in optimizing the cutting of tapes of various lengths into specific, predefined segment lengths. The application provides a user-friendly interface to enter tape lengths and calculates the most efficient combinations to meet the desired cutting requirements. The project has been implemented in two different languages, Python using PyQt5 and Rust using eframe/egui, to showcase the flexibility and performance of both.

---

## Key Features

- **Cross-Platform Compatibility**: Both Python (PyQt5) and Rust (eframe/egui) versions can be run on various operating systems.
- **User-Friendly Interface**: Simple and intuitive UI for adding, removing, and calculating tape lengths.
- **Flexible Tape Length Input**: Allows users to input multiple tape lengths and calculates the most efficient cutting combinations.
- **Efficient Calculation**: Implements an algorithm to find the best combination of cuts with specific ratios (e.g., 2:1, 5:2, 3:2).
- **Rich Text Output**: The output is formatted with color-coded segments to easily distinguish between different tape segments.
- **Python and Rust Implementations**: Provides both a Python version with PyQt5 for ease of use and a Rust version for high performance.

---

## Screenshots

![grafik](https://github.com/user-attachments/assets/29ed475a-1435-4ac7-a90c-503e04a91fa4)
![grafik](https://github.com/user-attachments/assets/835b0272-96d1-43d9-bfc8-2f134e7fec3a)


## Usage

### Python Version
1. **Requirements**:
   - Python 3.x
   - PyQt5

2. **Installation**:
   - Clone the repository: `git clone https://github.com/yourusername/tape-planner.git`
   - Navigate to the project directory: `cd tape-planner/python`
   - Install dependencies

3. **Run the Application**:
   - Execute the script: `python tape_planner.py`
   - The GUI will launch, allowing you to input tape lengths and calculate the best cutting strategy.

### Rust Version
1. **Requirements**:
   - Rust toolchain (install via `rustup`)

2. **Installation**:
   - Clone the repository: `git clone https://github.com/yourusername/tape-planner.git`
   - Navigate to the project directory: `cd tape-planner/rust`

3. **Run the Application**:
   - Build and run the application: `cargo run --release`
   - The application window will open, where you can input tape lengths and get the calculated results.

---

## Python and Rust Branches

- **Python Branch**: Focused on ease of use and rapid development. The Python version is implemented using PyQt5, making it accessible and easy to modify for developers familiar with Python.
  
- **Rust Branch**: Focused on performance and resource efficiency. The Rust version is built using the `eframe` and `egui` libraries, making it a great choice for production environments where performance is critical.

---

## Code Complexity

- **Python**:
  - The Python version leverages PyQt5 for the GUI, which simplifies the creation of the user interface but may require more memory and CPU resources compared to the Rust implementation.
  - The logic for calculating combinations of tape lengths is recursive and handles edge cases, such as specific predefined ratios (2:1, 5:2, 3:2).
  
- **Rust**:
  - The Rust implementation is designed to be highly performant, with parallel computation using the `rayon` library to maximize CPU usage during calculations.
  - The GUI is built with `eframe` and `egui`, which are more lightweight compared to PyQt5, resulting in a faster and more responsive application.

---

## Code Structure

### Python
- **Main Module** (`tape_planner.py`): Contains the GUI setup, logic for adding and removing tape lengths, and the core calculation algorithm.
- **Resource Management**: Handles loading the application icon and other resources needed for the UI.

### Rust
- **Main Application** (`main.rs`): Contains the GUI setup using `eframe` and the logic for managing tape lengths.
- **Parallel Computation**: Uses the `rayon` crate to parallelize the combination calculations, ensuring that the application makes full use of available CPU resources.

---

## Future Enhancements

- **Advanced Optimization Algorithms**: Implement more sophisticated algorithms to handle larger sets of input lengths and more complex ratios.
- **Multi-Platform Deployment**: Package both Python and Rust versions for easy installation on Windows, macOS, and Linux.
- **Additional Ratios**: Support additional custom ratios as per user input.
- **Save and Load Functionality**: Allow users to save their work and load previously saved tape length configurations.
- **Improved UI/UX**: Enhance the user interface with additional features like drag-and-drop for tape lengths, real-time updates, and better error handling.
- **Cloud Integration**: Option to store and retrieve tape configurations from the cloud, allowing users to access their plans from different devices.

---

This project is a great starting point for anyone interested in learning the differences in performance and complexity between Python and Rust while working on a real-world application. Contributions are welcome to both branches!
