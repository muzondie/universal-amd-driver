# Universal AMD Driver  

A Rust-based tool that simplifies AMD driver installation on Windows. It scans your system, identifies AMD components, and installs up-to-date drivers from official sources. Designed for users who want a hassle-free way to manage AMD drivers without searching manually.

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-amd-driver/releases) tab.  
2. Download the latest `.zip` file.  
3. Unzip the file and run `UniversalAmdDriver.exe`.  

## Usage  
1. **Run the app** (ensure you have administrator permissions).  
2. Wait for the automatic scan to detect your AMD hardware.  
3. Click **Install** to download and set up required drivers.  
4. Restart your system if prompted.  

## Features

### Core Functionality
- Automatic detection of AMD processors, graphics cards, and motherboard chipsets
- Driver version checking against AMD's official sources
- Silent installation for supported components (no manual input required)

### Technical Details
- Supports all AMD CPUs
- Compatible with Radeon RX series GPUs
- Handles X570/B550/A520 and other chipsets
- Background download manager with pause/resume capability
- Installation rollback on failure
- Detailed logs stored in `%AppData%\universal-amd-driver\logs`

### User Experience
- Clear progress indicators for scans/downloads/installs
- Hardware summary with component names and current driver versions
- Optional offline mode using existing driver packages
- Low resource usage (<100MB RAM, no background processes)

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-amd-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-amd-driver  
   cargo build --release  
   ```  
4. The executable will be in `target/release/`.  

## Contributing  
Contributions are currently closed due to limited maintenance capacity. Please check back later for updates.  

## License  
MIT License. See [LICENSE](LICENSE) for details.
