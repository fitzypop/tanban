default:
    echo 'Hello, world!'

publish:
    cargo package
    cargo publish
