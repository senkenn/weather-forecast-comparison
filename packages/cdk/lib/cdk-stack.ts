import * as cdk from "aws-cdk-lib";
import { Bucket, HttpMethods } from "aws-cdk-lib/aws-s3";
import type { Construct } from "constructs";

export class S3Stack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new Bucket(this, "MyBucket", {
      bucketName: "weather-forecast-comparison-data-store",
      cors: [
        {
          allowedMethods: [HttpMethods.GET],
          allowedOrigins: ["*"],
          allowedHeaders: ["*"],
        },
      ],
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });
  }
}
