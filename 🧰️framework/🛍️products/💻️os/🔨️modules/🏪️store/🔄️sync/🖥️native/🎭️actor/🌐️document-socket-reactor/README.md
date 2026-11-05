
## 🔌️ The process-wide I/O reactor every native document socket is driven by: one dedicated thread that parks on the platform I/O and time drivers, created on first use and never shut down.


🧭️ A caller that dials or awaits a hub document socket from a thread no runtime owns enters it with [`DocumentSocketReactor::enter`] for as long as it polls socket futures, or hands it a task with [`DocumentSocketReactor::spawn`]; nothing outside this module names the runtime behind it.
