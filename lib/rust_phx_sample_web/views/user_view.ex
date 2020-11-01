defmodule RustPhxSampleWeb.UserView do
  use RustPhxSampleWeb, :view

  def render("user.json", %{user: user}) do
    %{user: user}
  end
end
