# Multi-Agent Research Tasks

## Overview
Use OpenClaw's sub-agent system to parallelize Daggerheart rules research.

---

## Phase 1: Critical Path (Sequential)

### Agent 1: Dice Mechanics Researcher 🎲
**Priority**: CRITICAL (blocks everything else)

**Task**:
```
Research Daggerheart duality dice mechanics and document findings.

Goals:
1. Find and read official Daggerheart playtest materials
2. Document exact duality dice (2d12 Hope/Fear) mechanics
3. Identify critical success conditions (both 12s?)
4. Identify critical failure conditions (both 1s?)
5. Determine tie-breaking rules (when Hope == Fear)
6. Document modifier application rules
7. Research advantage/disadvantage mechanics
8. Find damage dice rules (d4, d6, d8, d10, d20)

Deliverable:
Update RESEARCH.md Section 1 (Duality Dice System) with verified facts.
Include source citations (page numbers, video timestamps).

Resources:
- https://www.daggerheart.com/ (download playtest PDF)
- https://drivethrurpg.com/ (search "Daggerheart")
- Daggerheart 101 video
- Community discussions (Reddit, Discord)

Format output as markdown suitable for RESEARCH.md
```

**Estimated time**: 30-45 minutes

---

## Phase 2: Parallel Research (After Phase 1 Complete)

### Agent 2: Character System Researcher 🧙

**Task**:
```
Research Daggerheart character creation and progression system.

Goals:
1. List all 9 character classes
2. Document attribute system (names, modifiers, ranges)
3. List available ancestries
4. Document character creation process
5. Research progression/leveling system (1-10? 1-20?)
6. Identify foundation abilities
7. Document heritage/community mechanics

Deliverable:
Update RESEARCH.md Section 2 (Character Attributes) and Section 3 (Classes).
Include examples from official materials.

Resources: Same as Agent 1
```

---

### Agent 3: Combat System Analyst ⚔️

**Task**:
```
Research Daggerheart combat mechanics and action economy.

Goals:
1. Document action economy (major/minor/reaction actions)
2. Research initiative system (if any)
3. Document attack roll resolution process
4. Identify damage calculation rules
5. Research armor/defense system
6. Document HP/Stress/Wounds tracking
7. List status conditions and effects
8. Research death and recovery mechanics

Deliverable:
Update RESEARCH.md Section 4 (Combat System) and Section 9 (Conditions).
Include combat examples.

Resources: Same as Agent 1
```

---

### Agent 4: Resources & Cards Specialist 🃏

**Task**:
```
Research Daggerheart Hope/Fear mechanics and domain card system.

Goals:
1. Document how Hope accumulates
2. Document how Fear accumulates
3. List what Hope can be spent on
4. List GM Fear move triggers
5. Research domain card structure (279 cards)
6. Document card acquisition and deck building
7. Research spell/ability activation mechanics
8. Identify resource refresh rules

Deliverable:
Update RESEARCH.md Section 5 (Hope & Fear) and Section 6 (Domain Cards).
Include specific card examples.

Resources: Same as Agent 1
```

---

## Execution Commands

### Start Phase 1:
```bash
# From main session, spawn dice mechanics researcher:
# Use sessions_spawn with the Agent 1 task above
```

### Start Phase 2 (after Phase 1 complete):
```bash
# Spawn all three remaining agents in parallel
# They'll work simultaneously and report back
```

### Consolidation:
```bash
# After all agents report:
# 1. Review all findings
# 2. Merge into RESEARCH.md
# 3. Resolve any conflicts/ambiguities
# 4. Update TDD_PLAN.md with verified test cases
# 5. Commit: "docs: update research with verified Daggerheart mechanics"
```

---

## Agent Coordination Strategy

### Main Session (You + Me):
- Orchestrate sub-agents
- Consolidate findings
- Resolve conflicts
- Make final decisions
- Update documentation
- Write tests based on verified rules

### Sub-Agents:
- Focus on assigned domain
- Research thoroughly
- Document findings
- Report back with structured output
- Don't implement code (research only)

---

## Success Criteria

Phase 1 complete when:
- [ ] Duality dice mechanics fully documented
- [ ] Critical success/failure rules verified
- [ ] Modifier application understood
- [ ] Damage dice rules documented

Phase 2 complete when:
- [ ] All 9 classes listed
- [ ] Attributes verified
- [ ] Combat action economy documented
- [ ] Hope/Fear mechanics explained
- [ ] Domain card system understood

Ready to implement when:
- [ ] All critical unknowns resolved
- [ ] RESEARCH.md sections updated
- [ ] Test cases written in TDD_PLAN.md
- [ ] Source citations included

---

## Fallback Plan

If sub-agents can't access web resources:
- You download PDF manually
- Share key excerpts in chat
- Agents analyze provided text
- Still faster than single-threaded research

---

## Timeline Estimate

- Phase 1 (Critical): 30-45 min
- Phase 2 (Parallel): 30-45 min (wall time, ~2 hours agent time)
- Consolidation: 15-30 min
- **Total**: ~90-120 minutes to complete research

Compare to sequential: 4-6 hours minimum!

---

Ready to start? 🚀
