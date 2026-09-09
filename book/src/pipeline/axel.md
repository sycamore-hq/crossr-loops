# AXEL — Execution Loop

**AXEL** = Automated eXecution Loop.

Skill: `axel` · Persona: `axel-conductor-agent`  
Plan-first: `plan-writer` → audit → Architect, then generate → mechanical → tester → reviewer (`graphs/code-gan.json`)

Normative detail: [HARNESS-SPEC.md §13](https://github.com/sycamore-hq/crossr-harness/blob/main/HARNESS-SPEC.md).

## Purpose

Drive **only AVRIL-blessed** (or explicitly human-authorized) PBIs through Plan → Execute → Test → Commit until acceptance criteria have evidence and the board is honest.

The conductor **never writes production code** — it sequences Generator + code GAN.

## Intake gate (non-negotiable)

AXEL starts only if one of:

1. AVRIL Blessed Backlog Summary lists the ids, or  
2. Board items carry a blessed marker (e.g. `avril-blessed`), or  
3. Human authorizes a finite PBI id set  

**Unblessed work is refused.** Scope changes return to [AVRIL](avril.md).

## Per-PBI loop

1. **Select** one ready PBI (deps done; prefer `pinto next`).
2. **Plan** — Generator + `plan-writer`; phases live inside the plan; blocking questions → stop.
3. Mechanical audit, then Architect at plan time. Three REJECTs → human. BLESS → commit the plan.
4. Board → **in-progress**.
5. Each phase: Generator → mechanical → Tester → Reviewer (each LLM gate must `BLESS`) → commit + tracking. Architect at code time only on an unsatisfiable claim. Each adversary receives an audited handoff packet and nothing else; after commit the conductor keeps the Completion Record and drops the review prose.
6. **AC evidence** — every checkbox needs recorded evidence; verification matrix green.
7. Board → review → **done** only when AC complete.
8. PBI Completion Record → next or stop.

## Activation

> Using `axel` + `gan-verdict` to execute the next blessed PBI through PETC until AC are evidenced and the board is honest. Language/domain skills are disclosed to subagents; the conductor never loads them.

## Optional: Mitchell decomposition mode

**Default off.** When enabled (“mitchell” / decomposition mode / harness `decomposition_mode`), the plan Architect rejects a phase whose stated size exceeds T (default **1500**). `audit-plan --loc-threshold T` is the mechanical compare. A code-time measure (`git diff --numstat`, added+deleted) over T does **not** commit: halt and return to the plan with superseding claims. Nobody splits at code time. Does not bypass intake or code GAN.

Normative contract: [mitchell-decomposition-contract.html](https://github.com/scull7/crossr-skills/blob/main/docs/plans/mitchell-decomposition-contract.html) (issue #43).
