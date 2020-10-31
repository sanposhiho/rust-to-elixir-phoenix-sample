defmodule RustPhxSampleWeb.SampleView do
  use RustPhxSampleWeb, :view

  def render("sample.json", %{number: num}) do
    %{number: num}
  end
end
