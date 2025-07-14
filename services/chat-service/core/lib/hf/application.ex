defmodule Hf.Application do
  use Application

  def start(_type, _args) do
    dispatch = :cowboy_router.compile([
      {:_, [
        {"/ws", Hf.WebSocketHandler, []}
      ]}
    ])

    children = [
      {Registry, keys: :duplicate, name: Hf.Registry},
      %{
        id: :http_listener,
        start: {
          :cowboy,
          :start_clear,
          [
            :http,
            [port: 4000],
            %{env: %{dispatch: dispatch}}
          ]
        }
      }
    ]

    opts = [strategy: :one_for_one, name: Hf.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
