# <p align="center">OBJEKTDB</p>
<p align="center"><i>Effortless data management for innovative Rust applications</i></p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white" />
</p>

<p align="center">
  <img src="img/banner.png" alt="objektDB banner" />
</p>

---
![Status](https://img.shields.io/badge/status-in%20development-yellow?style=for-the-badge)

### 🚧 Project Status  
> ⚠️ **ObjektDB is currently under development**  
> It is not ready for production use yet.

## Overview

**objektDB** is a lightweight, embedded object-oriented database for Rust. It allows you to persist structured objects directly in files, without relying on external database servers. Its focus is on simplicity and ease of use, making it perfect for small applications, prototypes, or learning Rust database management.

---

## Why objektDB?

- 🗄️ **Embedded & Lightweight:** No external dependencies; everything runs inside your application.  
- 📦 **Direct Object Storage:** Store and retrieve structs easily.  
- ⚡ **Quick Setup:** Start using it in minutes, ideal for rapid prototyping.  
- 🛠️ **Simple API:** Minimal boilerplate and easy integration into Rust projects.  

---

## Getting Started

### Prerequisites

- **Rust**  
- **Cargo**  

### Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
objektdb = "0.1"
```
If you want to insert and register the methods of an impl block for a struct, you need to insert the following feature:
```toml
[dependencies]
objektdb = {version="0.1", features=["impl_blocks"]}
```
Warning. If you enable this feature, every struct that uses the `Objekt` macro must have an impl block that applies the `objekt_impl` macro, event if it's empty.

Import it in your Rust code:

```rust
use objektdb::{
  Objekt,
  odb,
  Set,
  from_bytes::FromBytes,
  objekt_impl
};
```

### Usage Example

```rust
todo!()
```

For other examples and tutorials(which refer to and explain the code of the examples) you can go to the [examples folder](examples) or the [tutorials folder](docs/tutorials/)(Not yet ready)

---

## Restrictions

* Maximum of **255 tables** per database.
* Struct names must not exceed **64 characters**.
* Interaction is only via the **provided macros and trait functions**; no dedicated query language yet.
* **Relationships between objects are not supported at the moment.**
* Supported types: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool, char, string, usize, isize.

---

## New Version

* **DB file template version:** 1

