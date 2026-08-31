# Status Dashboard (In-Harness UI)

Parameterized for AXEL: the source of truth noun is the **board**. AVRIL's copy (`avril/references/status-dashboard.md`) uses **plan**. Same law, different checkpoint list.

You keep a live view of the work so a human can see progress without reading the
transcript. The dashboard renders **completed / in progress / todo** from the
harness's own tracking artifacts — it is generated, never hand-written, and it is
never the source of truth: the board and tracking artifacts are.

**Refresh it at every checkpoint below**, immediately after the tracking artifact
or board changes — not in a batch at the end:

- At session start, after the pre-flight read.
- On every board transition you make: → in-progress, → review, → done.
- After each phase earns its three code-GAN BLESS marks.
- At the acceptance-criteria evidence gate, before you allow `done`.
- At the PBI Completion Record, before advancing to the next PBI.

The refresh command is a harness parameter disclosed at activation (in this
repository: `just status` for the terminal view, `just status-html` to also write
the HTML dashboard). If the harness discloses no dashboard command, say so once
and continue — never hand-write a dashboard file, and never fake a status you
have not read from the board or tracking artifacts.

Commit the generated HTML **only at phase or PBI boundaries**, not on every
refresh, so the diff stays meaningful.
