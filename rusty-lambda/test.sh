echo "Status:"
aws lambda invoke --function-name RustyLambda --payload '{"name": "Rust Rome"}' output.json

echo "Response"
cat output.json

