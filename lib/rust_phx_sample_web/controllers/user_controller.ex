defmodule RustPhxSampleWeb.UserController do
  use RustPhxSampleWeb, :controller
  use Rustler, otp_app: :rust_phx_sample, crate: :rustphxsampleweb_usercontroller

  def create(conn, %{"user" => %{"name" => name, "age" => age}}) do
    {:ok, {id, name, age}} = create_user(name, age)

    user = %{
      id: id,
      name: name,
      age: age,
    }

    render(conn, "user.json", user: user)
  end

  def create_user(_name, _age), do: exit(:nif_not_loaded)
end
