defmodule RustlerExampleGenServer do
  @moduledoc """
  Documentation for `RustlerExampleGenServer`.
  """

  alias RustlerExampleGenServer.Native

  use GenServer

  defdelegate init(args), to: Native
  defdelegate handle_call(request, from, state), to: Native

  def get(pid, key) do
    GenServer.call(pid, {:get, key})
  end

  def put(pid, key, value) do
    GenServer.call(pid, {:put, key, value})
  end
end
