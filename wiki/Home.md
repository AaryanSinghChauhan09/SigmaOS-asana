# SigmaOS-asana

SigmaOS integration component for **Asana tasks**.

## Commands

```bash
sigmaos-asana install   # install package + deploy defaults
sigmaos-asana deploy    # copy config templates to $HOME
sigmaos-asana status    # binary/package/config report
sigmaos-asana reset     # remove deployed config
sigmaos-asana open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
