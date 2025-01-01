# RythmMV App [Windows]

RythmMV is a Windows application built using **Rust** and **Tauri**. This guide will help you compile the application on your local machine.

---

## 🚀 Steps to Compile

### **Step 1: Install Rust**
Ensure you have **Rust** installed. If not, download it from the official [Rust Website](https://www.rust-lang.org/tools/install). Follow the instructions to set it up.

---

### **Step 2: Clone the Repository**
Clone the RythmMV repository using the following command:

```bash
git clone https://github.com/RythmMV/RythmMV-App.git
```

---

### **Step 3: Navigate to the Directory**
Move into the project directory:

```bash
cd RythmMV-App
```

---

### **Step 4: Open PowerShell**
Open a **PowerShell** terminal inside the `RythmMV-App` folder.

---

### **Step 5: Install Tauri CLI**
Run the following command to install the Tauri CLI, which is required to build the app:

```bash
cargo install tauri-cli
```

Wait for the installation to complete.

---

### **Step 6: Build the App**
Build the application by running:

```bash
cargo tauri build
```

Once the build process is complete, you’ll find the executable file (`rythm.exe`) in one of the following directories within the project folder:

- `src-tauri/target/release/`
- `src-tauri/target/debug/`

That's it! 🎉 Your RythmMV app is now ready to use.

---

## 🛠️ Prerequisites
- Rust (latest version)
- Tauri CLI (installed via `cargo`)

---

## 📂 Repository
Visit the [RythmMV GitHub Repository](https://github.com/RythmMV/RythmMV-App) for updates and contributions.

---

## 💡 Notes
- Ensure all dependencies are properly installed before building the app.
- For any issues, please check the [Rust Documentation](https://doc.rust-lang.org/) and [Tauri Documentation](https://tauri.app/).

---

Happy coding! 🎶
