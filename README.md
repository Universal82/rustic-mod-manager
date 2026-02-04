# This is the Rustic (Skyrim) Mod Manager!

### Why does this exist if MO2 and Vortex exist?
I'm a Linux user, and those apps are for Windows! Yes, there is the [Linux Mod Organizer 2 installer](https://github.com/Furglitch/modorganizer2-linux-installer), however, it doesn't work with my Arch configuration and I couldn't be bothered to figure out why. So I'm making my own!

### Will this be better than other implementations?
Probably not. I'm a hobbyist, not a professional. I just mess around and do what works and feels right. I don't do my research, but if you know something I don't, feel free to make a pull request! If you do, I'd also like to be told why your way is better.

### How does/will this work?
From what I know Proton follows symlinks. So I think I can install mods to a mods directory, link their data (and the base game's data) to an instance's directory with respect to load order, link the game's executable into the instance's directory, and run it through there. Optimistically, that should be all that's needed.

### Why are you not using crate X?
Like I said, I don't really do my research. The only dependencies I know are pretty basic and general. I haven't looked very much into more niche things because I'm still figuring stuff out. If you have a good crate that you think would help me in developing this, suggest it in an issue. I think there's an enhancement category or something like that.
