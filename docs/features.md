# Game specification

## V0.1 feature list

- First Person Shooter
- Inventory
- Single Player proof of concept first, feel my way into multiplayer later

### Floating Gun
Possibly include some arms, cba with reload animations for the moment, back seat that shit.

### AI NPCs
None planned

### Baked Animations
None planned

### Player
- Health
- Death
- Spawn system
- Walk sway
- Aim Down Sight with smooth interuptable procedural animation that returns to center properly and restarts from center when ADS'd again
- Movement with lean, jump and gravity
- Collisions

### Weapon
- Build in some weapon specific stats
- Weapon sway (low priority)
- Aim Down Sight must line up with the sight and potentially work with different sights
- Damage can be changed per weapon or even on the same weapon, like when using different ammo (weapon mods/attachments have stat modifiers?)
- Weapons are spawnable, and work with the inventory system
- Shooting - spawns projectile that has it's own collision detection (future potential for bullet module that takes over but that's maybe over-egging it)
- recoil and bullet spread
- shot MOA

### Inventory
- Basic Tarkov-like XY stacking system
- Ability to open other containers and view/take/give items

### Advanced Inventory
- Z Stacking / Grouping
- Multiplayer enabled inventory sharing (look at player inventory as if player is a container)

### Map
- Super basic map tha has some basic things to hide behind and is lit properly

## V0.2 feature list

- Enabling multiplayer

### Multiplayer Server
- Written in Rust, of course
- No need to be async
- GRPC
- Accept calls and broadcast them to everyone (client authority)

### Advanced Multiplayer Server (maybe later)
- Build in some form of server authority

### Multiplayer Mode
Basic death match and simple basic UI is all I need
