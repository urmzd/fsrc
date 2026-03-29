from flask import Flask, jsonify

app = Flask(__name__)

@app.route("/health")
def health():
    return jsonify(status="ok")

@app.route("/api/users")
def list_users():
    return jsonify(users=[])

if __name__ == "__main__":
    app.run(port=8080)
