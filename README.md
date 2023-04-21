### Structure
```
api
├── Cargo.lock
├── Cargo.toml
├── data
│   ├── dinosaurs
│   └── enron-emails
└── src
    ├── dinos
    │   └── dinos.rs
    ├── enron
    │   └── enron.rs
    ├── fairings.rs
    ├── lib.rs
    ├── location
    │   └── location.rs
    ├── main.rs
    ├── metrics
    │   ├── metrics.rs
    │   └── visitors.rs
    ├── root
    │   └── root.rs
    └── slow
        └── slow.rs
```

### Development
```bash
git clone https://github.com/ma1ted/api
cd api
git submodule update --init --recursive
```
