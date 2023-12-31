class TerminalGuitarTuner < Formula
  version '0.1.0'
  desc "A simple guitar tuner in your terminal."
  homepage "https://github.com/Goose97/terminal-guitar-tuner"
  url "https://github.com/Goose97/terminal-guitar-tuner/releases/download/v#{version}/terminal-guitar-tuner_#{version}_aarch64-apple-darwin.tar.gz"
  sha256 "5c9f890f04695c97f7b932c33abba973aa8a10a06c84be041e687970974cf6c5"

  def install
    bin.install "guitar-tuner"
  end
end
