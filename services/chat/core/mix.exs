defmodule HF.Chat.MixProject do
  use Mix.Project

  def project do
    [
      app: :core,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Hf.Application, []},
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:cowboy, "~> 2.9"},
      {:plug_cowboy, "~> 2.6"}
    ]
  end
end
