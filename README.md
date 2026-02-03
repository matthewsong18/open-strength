# Open Strength

> **Zero bloat. Zero ads. 100% Iron.**\
> An open-source, privacy-first, strength training tracker.

## Manifesto

This project aims to fill a specific gap in the weightlifting app market. An app
that will just let you log your training. That's it.

Existing solutions usually fall into two traps:

1. **Unverifiable Privacy**: Great apps exist out there that check all the
   boxes, but are closed-source.
2. **Bloat**: Open-source alternatives succumb to feature-creep, adding social
   feeds, AI features, and forced community interactions.

**Open Strength** aims to just do one job. Track strength training. Make your
routines, add any equipment/machine/workout you want, all offline.

### Feature List:

- **Unlimited Flexibility**: Create any routine with any equipment.
- **Universal Tracking**: Track sets, reps, and volume in lbs or kgs.
- **Offline First**: Zero network dependency. The app works in whatever basement
  gym you can find.
- **Private by Design:** Data is stored locally and encrypted at rest.
- **Open Source:** Licensed under **GNU AGPLv3**.

### Anti List:

- **No AI**
- **No Ads**
- **No Community**
- **No Cloud**

## How?

### How will this be more private than the alternatives?

Unlike "privacy-focused" cloud apps, Open Strength is **Local-First**.

- **No Network Requests:** The mobile adapter has no permission to access the
  internet.
- **Encryption at Rest:** Data is encrypted locally, ensuring that even if
  physical access to the device is compromised, the data remains unreadable.
- **Verifiable:** Because the code is open source, these claims can be audited
  by anyone.

### How will feature-creep be prevented?

The application is built using **Hexagonal Architecture**.

- **Core Isolation:** The business logic (Routines/Sets) is strictly isolated
  from the UI and device details.
- **Strict Scope:** New features are only considered if they serve the core loop
  (Plan -> Lift -> Log).
- **Extension over Modification:** Niche features will be handled via modular
  adapters rather than bloating the core domain. Feel free to fork & add your
  own adapter to suit your needs.
