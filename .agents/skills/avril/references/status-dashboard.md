# Status Dashboard (In-Harness UI)

Parameterized for AVRIL: the source of truth noun is the **plan** (board / backlog). AXEL's copy (`axel/references/status-dashboard.md`) uses **board**. Same law, different checkpoint list.

You keep a live view of the work so a human can see progress without reading the
transcript. The dashboard renders **completed / in progress / todo** from the
harness's own tracking artifacts — it is generated, never hand-written, and it is
never the source of truth: the plan and tracking artifacts are.

**Refresh it at every checkpoint below**, immediately after the tracking artifact
or plan changes — not in a batch at the end:

- At session start, before proposing anything (establishes the baseline).
- After each batch of proposed PBIs lands in the board or backlog file.
- After each PBI reaches unanimous BLESS.
- At the planning stop, alongside the Blessed Backlog Summary.

The refresh command is a harness parameter disclosed at activation (in this
repository: `just status` for the terminal view, `just status-html` to also write
the HTML dashboard). If the harness discloses no dashboard command, say so once
and continue — never hand-write a dashboard file, and never fake a status you
have not read from the plan or tracking artifacts.

Commit the generated HTML **only at phase or PBI boundaries**, not on every
refresh, so the diff stays meaningful.
