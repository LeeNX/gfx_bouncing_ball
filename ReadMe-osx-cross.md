# OSX aarch for x86_64
```bash
# Install brew x86_64
arch -x86_64 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# install x86_64 brew libs
arch -x86_64 /usr/local/bin/brew install sdl2 sdl2_ttf freetype sdl2_image
```
