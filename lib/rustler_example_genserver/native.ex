defmodule RustlerExampleGenServer.Native do
  use Rustler, otp_app: :rustler_example_genserver, crate: "rustler_example_genserver"

  # When your NIF is loaded, it will override this function.
  def init(_args), do: :erlang.nif_error(:nif_not_loaded)
  def handle_call(_request, _from, _state), do: :erlang.nif_error(:nif_not_loaded)

  def put(_state, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def get(_state, _key), do: :erlang.nif_error(:nif_not_loaded)
end
