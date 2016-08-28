var Widget = React.createClass({
  render: function() {
    return (
      <div className="widget">
        <h2>Welcome, world!</h2>
      </div>
    );
  }
});

ReactDOM.render(
  <Widget />,
  document.getElementById('content')
);

