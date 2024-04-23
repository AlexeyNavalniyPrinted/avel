### First of all, docker, kubectl, helm and python got to be installed to deploy this project

```bash
python auto.py
```
### And well, practically speaking, the deployment is over
### The only thing that left is to connect prometheus to grafana
To add prometheus to grafana, you need to log in grafana as shown below.
Add a data source and set the URL to http://prometheus-operated.default.svc.cluster.local:9090.
Then create a visualization, and it got to be working

# How to access to resources?

```bash
# Access cockroach db as root
kubectl wait --for=condition=ready --timeout=600s pod/cockroachdb-client-secure
kubectl exec -it cockroachdb-client-secure -- cockroach sql --certs-dir=./cockroach-certs --host=cockroach-cockroachdb-public 
```

```bash
# localhost:8080 / roach:password / Access cockroachdb admin panel
kubectl port-forward service/cockroach-cockroachdb-public 8080 
```

```bash
# localhost:9090 / Access prometheus admin panel
kubectl port-forward prometheus-cockroachdb-0 9090 
```

```bash
# Windows (PowerShell) command
kubectl get secret grafana -o jsonpath="{.data.admin-password}" | ForEach-Object { [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String($_)) }
# Linux command
kubectl get secret grafana -o jsonpath="{.data.admin-password}" | base64 --decode ; echo
# localhost:3000 / admin:password_from_command_above
kubectl port-forward deployment/grafana 3000 
```

# CI CD section

Run an auto script with a pipeline parameter set to true

## Sonarqube

### Github 

Go to https://github.com/settings/apps and create a new app

Get App ID, Client id, generate client secret and private key from https://github.com/settings/apps/{name}

Install the application in Repository

### Creating Sonarqube

```bash
# localhost:9000 / admin:admin
kubectl port-forward pod/sonarqube-sonarqube-0 9000 
```

Login and if asked, change the password. Choose GitHub  

Configuration name: any name

Github API url: https://api.github.com/ 

Then fill the rest with asked data 

Import Repository

Use the global settings

Then go to Project Information and copy Project Key

## Jenkins

```bash
kubectl exec --namespace default -it svc/jenkins -c jenkins -- /bin/cat /run/secrets/additional/chart-admin-password
# localhost:8080 / admin:password_from_command_above
kubectl port-forward svc/jenkins 8080:8080 
```

### Preparations

Go to: manage Jenkins -> Plugins -> Available Plugins -> Docker Pipeline / Install plugin

Go to: manage Jenkins -> Credentials -> global -> Add credentials 

### Credentials

First credentials:

Kind: Username with password

Username: DockerHub username

Password: DockerHub password

Id: dockerhub-credentials

Second secret:

Kind: Secret text

Secret: Sonarqube Project Key

Id: sonar-project-key

### Pipeline

Then go to New Item, choose Pipeline and enter any name

Configure Pipeline and test it with Build Now button

