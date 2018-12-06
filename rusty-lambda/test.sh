echo "Status:"
aws lambda invoke --function-name RustyLambda --payload '{"firstName": "Rust Roma"}' output.json

echo "Response"
cat output.json
echo ""

