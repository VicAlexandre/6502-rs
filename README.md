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

- **Instructions**: Implemented. The core instruction set of the 6502 is under development, with the aim to cover all official opcodes and behaviors.

- **TUI (Text-based User Interface)**: Implemented. A real-time interface that will display and update the current status of the processor with each execution step. This will enhance the usability for educational purposes and debugging.

- **Testing**: Currently in progress.

## Contributing

We welcome contributions from fellow students and enthusiasts. Please feel free to fork the repository, make your changes, and submit a pull request.

## Implemented Instructions
| Instruction | Opcode | Addressing Mode | Implemented | Tested |
|-------------|--------|-----------------|-------------|--------|
| BRK         | 00     | impl            | ✅          |  ✅      |
| ORA         | 01     | X,ind           | ✅          |   ✅     |
| ORA         | 05     | zpg             | ✅          |  ✅      |
| ORA         | 09     | #               | ✅          |  ✅       |
| ORA         | 0D     | abs             | ✅          |   ✅     |
| ORA         | 15     | zpg,X           | ✅          |   ✅     |
| ORA         | 19     | abs,Y           | ✅          |   ✅     |
| ORA         | 1D     | abs,X           | ✅          |   ✅     |
| ASL         | 0A     | impl            | ✅          |   ✅     |
| ASL         | 06     | zpg             | ✅          |   ✅     |
| ASL         | 0E     | abs             | ✅          |     ✅   |
| ASL         | 16     | zpg,X           | ✅          |   ✅     |
| ASL         | 1E     | abs,X           | ✅          |  ✅      |
| PHP         | 08     | impl            | ✅          |   ✅     |
| BPL         | 10     | rel             | ✅          |   ✅     |
| CLC         | 18     | impl            | ✅          |   ✅     |
| JSR         | 20     | abs             | ✅          |   ✅     |
| AND         | 21     | X,ind           | ✅          | ✅        |
| AND         | 25     | zpg             | ✅          | ✅        |
| AND         | 29     | #               | ✅          | ✅        |
| AND         | 2D     | abs             | ✅          | ✅        |
| AND         | 31     | ind,Y           | ✅          | ✅      |
| AND         | 35     | zpg,X           | ✅          | ✅      |
| AND         | 39     | abs,Y           | ✅          | ✅      |
| AND         | 3D     | abs,X           | ✅          | ✅      |
| BIT         | 24     | zpg             | ✅          | ✅     |
| BIT         | 2C     | abs             | ✅          | ✅     |
| ROL         | 2A     | impl            | ✅          | ✅     |
| ROL         | 26     | zpg             | ✅          | ✅     |
| ROL         | 2E     | abs             | ✅          | ✅     |
| ROL         | 36     | zpg,X           | ✅          | ✅     |
| ROL         | 3E     | abs,X           | ✅          | ✅     |
| PLP         | 28     | impl            | ✅          | ✅     |
| BMI         | 30     | rel             | ✅          | ✅     |
| SEC         | 38     | impl            | ✅          |  ✅    |
| RTI         | 40     | impl            | ✅          |        |
| EOR         | 41     | X,ind           | ✅          | ✅     |
| EOR         | 45     | zpg             | ✅          | ✅     |
| EOR         | 49     | #               | ✅          | ✅     |
| EOR         | 4D     | abs             | ✅          | ✅     |
| EOR         | 51     | ind,Y           | ✅          | ✅     |
| EOR         | 55     | zpg,X           | ✅          | ✅     |
| EOR         | 59     | abs,Y           | ✅          | ✅     |
| EOR         | 5D     | abs,X           | ✅          |✅      |
| LSR         | 4A     | impl            | ✅          |✅      |
| LSR         | 46     | zpg             | ✅          |✅      |
| LSR         | 4E     | abs             | ✅          |✅      |
| LSR         | 56     | zpg,X           | ✅          |✅      |
| LSR         | 5E     | abs,X           | ✅          |✅      |
| PHA         | 48     | impl            | ✅          | ✅     |
| JMP         | 4C     | abs             | ✅          | ✅     |
| JMP         | 6C     | ind             | ✅          | ✅     |
| BVC         | 50     | rel             | ✅          |      |
| CLI         | 58     | impl            | ✅          |      |
| RTS         | 60     | impl            | ✅          |      |
| ADC         | 61     | X,ind           | ✅          | ✅      |
| ADC         | 65     | zpg             | ✅          | ✅  |
| ADC         | 69     | #               | ✅          | ✅  |
| ADC         | 6D     | abs             | ✅          | ✅     |
| ADC         | 71     | ind,Y           | ✅          | ✅     |
| ADC         | 75     | zpg,X           | ✅          | ✅    |
| ADC         | 79     | abs,Y           | ✅          | ✅     |
| ADC         | 7D     | abs,X           | ✅          | ✅     |
| ROR         | 6A     | impl            | ✅          |        |
| ROR         | 66     | zpg             | ✅          |        |
| ROR         | 6E     | abs             | ✅          |        |
| ROR         | 76     | zpg,X           | ✅          |        |
| ROR         | 7E     | abs,X           | ✅          |        |
| PLA         | 68     | impl            | ✅          |        |
| BVS         | 70     | rel             | ✅          |        |
| SEI         | 78     | impl            | ✅          |        |
| STA         | 81     | X,ind           | ✅          |        |
| STA         | 85     | zpg             | ✅          |        |
| STA         | 8D     | abs             | ✅          |        |
| STA         | 91     | ind,Y           | ✅          |        |
| STA         | 95     | zpg,X           | ✅          |        |
| STA         | 99     | abs,Y           | ✅          |        |
| STA         | 9D     | abs,X           | ✅          |        |
| STY         | 84     | zpg             | ✅          |        |
| STY         | 8C     | abs             | ✅          |        |
| STY         | 94     | zpg,X           | ✅          |        |
| STX         | 86     | zpg             | ✅          |        |
| STX         | 8E     | abs             | ✅          |        |
| STX         | 96     | zpg,Y           | ✅          |        |
| DEY         | 88     | impl            | ✅          |        |
| TXA         | 8A     | impl            | ✅          |        |
| TYA         | 98     | impl            | ✅          |        |
| TXS         | 9A     | impl            | ✅          |        |
| LDY         | A0     | #               | ✅          |        |
| LDY         | A4     | zpg             | ✅          |        |
| LDY         | AC     | abs             | ✅          |        |
| LDY         | B4     | zpg,X           | ✅          |        |
| LDY         | BC     | abs,X           | ✅          |        |
| LDA         | A1     | X,ind           | ✅          |        |
| LDA         | A5     | zpg             | ✅          |        |
| LDA         | A9     | #               | ✅          | ✅        |
| LDA         | AD     | abs             | ✅          |        |
| LDA         | B1     | ind,Y           | ✅          |        |
| LDA         | B5     | zpg,X           | ✅          |        |
| LDA         | B9     | abs,Y           | ✅          |        |
| LDA         | BD     | abs,X           | ✅          |        |
| LDX         | A2     | #               | ✅          | ✅     |
| LDX         | A6     | zpg             | ✅          |        |
| LDX         | AE     | abs             | ✅          |        |
| LDX         | B6     | zpg,Y           | ✅          |        |
| LDX         | BE     | abs,Y           | ✅          |        |
| TAY         | A8     | impl            | ✅          |        |
| TAX         | AA     | impl            | ✅          |        |
| TSX         | BA     | impl            | ✅          |        |
| CPY         | C0     | #               | ✅          |        |
| CPY         | C4     | zpg             | ✅          |        |
| CPY         | CC     | abs             | ✅          |        |
| CMP         | C1     | X,ind           | ✅          |        |
| CMP         | C5     | zpg             | ✅          |        |
| CMP         | C9     | #               | ✅          |        |
| CMP         | CD     | abs             | ✅          |        |
| CMP         | D1     | ind,Y           | ✅          |        |
| CMP         | D5     | zpg,X           | ✅          |        |
| CMP         | D9     | abs,Y           | ✅          |        |
| CMP         | DD     | abs,X           | ✅          |        |
| DEC         | C6     | zpg             | ✅          |        |
| DEC         | CE     | abs             | ✅          |        |
| DEC         | D6     | zpg,X           | ✅          |        |
| DEC         | DE     | abs,X           | ✅          |        |
| INY         | C8     | impl            | ✅          |        |
| DEX         | CA     | impl            | ✅          |        |
| BNE         | D0     | rel             | ✅          |   ✅     |
| CLD         | D8     | impl            | ✅          |        |
| CPX         | E0     | #               | ✅          |        |
| CPX         | E4     | zpg             | ✅          |        |
| CPX         | EC     | abs             | ✅          |        |
| SBC         | E1     | X,ind           | ✅          |        |
| SBC         | E5     | zpg             | ✅          |        |
| SBC         | E9     | #               | ✅          |        |
| SBC         | ED     | abs             | ✅          |        |
| SBC         | F1     | ind,Y           | ✅          |        |
| SBC         | F5     | zpg,X           | ✅          |        |
| SBC         | F9     | abs,Y           | ✅          |        |
| SBC         | FD     | abs,X           | ✅          |        |
| INC         | E6     | zpg             | ✅          |        |
| INC         | EE     | abs             | ✅          |        |
| INC         | F6     | zpg,X           | ✅          |        |
| INC         | FE     | abs,X           | ✅          |        |
| INX         | E8     | impl            | ✅          |        |
| NOP         | EA     | impl            | ✅          |        |
| BEQ         | F0     | rel             | ✅          |        |
| SED         | F8     | impl            | ✅          |        |

### Confused about how addressing modes work? Understand them better [here](https://www.nesdev.org/obelisk-6502-guide/addressing.html).

## License

This project is licensed under the GPL-3.0 license. For more information, please refer to the [LICENSE](LICENSE) file.
