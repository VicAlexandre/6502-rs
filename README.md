# 6502 Emulator

This repository contains a 6502 microprocessor emulator developed in Rust as a hobby project. It is created and maintained by students from the Federal University of Alagoas (UFAL).

## Developers

- **Victor Miranda**
  - Email: [varm@ic.ufal.br](mailto:varm@ic.ufal.br)
  - Computer Science Undergrad Student

- **Vinicius Teixeira**
  - Email: [vtpr@ic.ufal.br](mailto:vtpr@ic.ufal.br)
  - Computer Engineering Undergrad Student

## Project Overview

This emulator aims to accurately simulate the operation of the 6502 microprocessor, a significant piece in the history of computing used in various classic systems. It is designed to work in a step-by-step fashion, allowing the user to observe the internal state of the processor at each instruction executed, making it a valuable tool for educational purposes and debugging. Additionally, it will display the current status of the processor in real-time through a TUI (Text-based User Interface).

## Roadmap

- **Instructions**: Currently being implemented. The core instruction set of the 6502 is under development, with the aim to cover all official opcodes and behaviors.

- **TUI (Text-based User Interface)**: Planned feature. A real-time interface that will display and update the current status of the processor with each execution step. This will enhance the usability for educational purposes and debugging.

## Contributing

We welcome contributions from fellow students and enthusiasts. Please feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the GPL-3.0 license. For more information, please refer to the [LICENSE](LICENSE) file.

## Implemented Instructions
- BRK [x]
- CLC [x]
- CLD [x]
- CLI [x]
- CLV [x]
- NOP [x]
- INY [x]
- INX [x]
- LDA [x]
- LDX [x]
- LDY [x]
- LSR [x]
- ORA [x]
- PHA [x]
- PHP [x]
- PLA [x]
- ROL [x]
- ASL [x]
- ADC []
- AND []
- BCC []
- BCS []
- BEQ []
- BIT []
- BMI []
- BNE []
- BPL []
- BVC []
- BVS []
- CMP []
- CPX []
- CPY []
- DEC []
- DEX []
- DEY []
- EOR []
- INC []
- JMP []
- JSR []
- LSR []
- ROR []
- RTI []
- RTS []
- SBC []
- SEC []
- SED []
- SEI []
- STA []
- STX []
- STY []
- TAX []
- TAY []
- TSX []
- TXA []
- TXS []
- TYA []