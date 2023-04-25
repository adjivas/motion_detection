FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main

# Change the packages to your dependencies.
RUN install_deb.sh x86_64 libgstreamer1.0-dev \
  libgstreamer-plugins-base1.0-dev \
  libssl-dev
